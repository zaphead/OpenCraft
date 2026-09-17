# OpenCraft

This repo is a from-scratch voxel game in Rust: our engines, our protocol, Java-like 20-tick feel, native Mac now and Windows later. Leftover files in the tree are not authority; this document is. The block roster, crafting recipes, worldgen, and other game content are **not** specified here — they get their own design spec next.

## Agent law

This file is binding until the user approves a change to it.

1. Read this file before doing work. Follow it.
2. If a user request fights this file, do not silently violate it. Advise: say what collides, what it would cost, and what change to this file would be required. Then wait for approval.
3. Agents may argue that a rule is wrong. Agents may not ignore a rule.
4. Do not over-specify downstream content in this file. Do not invent crates, deps, patterns, or backends "just this once."
5. New crates, new allowed deps, new threads, new backends, or a new data model require an AGENTS.md edit **and** user approval.
6. Prefer deleting and simplifying over adding. Do not preserve leftover/legacy code as compatibility.

## System shape

```text
                    +------------------+
                    |  dedicated bin   |
                    |     server       |
                    +--------+---------+
                             | protocol
                             v
+------------+      +--------+---------+      +----------------+
| window/OS  |      |  client bin      |      |  audio device  |
|   winit    |----->|  predict + interp+----->|  cpal output   |
+------------+      |  can EMBED server|      +----------------+
                    +---+-----+----+---+
                        |     |    |
                        v     v    v
                   render   input  embedded server
                   (wgpu    (our   (same crate as
                    rented)  types) dedicated)
```

**Two binaries, one sim.** `server` is always the authority. Singleplayer and listen-server **embed that same server** in-process. Dedicated play is the same code in another process. There is no second implementation of game rules.

```text
client  --> engine-render, engine-audio, engine-input, protocol, game, world, server
server  --> protocol, game, world, engine-phys, engine-core
game    --> world, protocol, engine-phys, engine-core
world   --> engine-phys, engine-core
engine-render  --> engine-core     (wgpu stays private)
engine-audio   --> engine-core     (cpal stays private)
engine-input   --> engine-core
engine-phys    --> engine-core
protocol       --> engine-core
engine-core    --> (std + allowed math/infra only)
```

`server` must never depend on `engine-render`, `engine-audio`, `winit`, `wgpu`, or `cpal`. Game rules must never depend on GPU/audio/window types.

## Architectural invariants

These are not taste. Breaking them is a failed change.

1. **Spiritual clone, not a wrapper.** Custom protocol. Custom world saves. No outside net protocols, no foreign world formats.
2. **Server is god.** 20 TPS, 50 ms ticks. The client predicts the local player, interpolates everyone else, and applies server corrections. Clients do not authoritatively place/break/move.
3. **Body clock.** Movement, drag, accel, sneak/sprint, jump, and AABB stepped collision reimplement **publicly documented physics knowledge**. Then we feel-test. No decompiled code.
4. **wgpu is a rental.** `engine-render` exposes _our_ command/resource API (buffers, textures, meshes, draw lists, frame submit). wgpu (later Metal/DX12/Vulkan) is a private backend. `game`, `world`, `server`, and `client` gameplay code never see wgpu types.
5. **cpal/winit are rentals too.** Same isolation: our input events, our mixer API, our surface/window handoff. Platform types stay in the platform-facing crate/module that owns them.
6. **Chunks are not entities.** 16×16×16 paletted sections stacked into chunks. World height is a later game spec. Entities (players, mobs, items) are a tiny custom store: generational IDs + SoA components. No general ECS, no hecs/bevy, no chunks-as-entities.
7. **Thread graph is fixed.** Render thread (uncapped FPS). Server/sim thread (20 TPS). Audio thread. Chunk-gen + mesh worker pool. No sim on the render thread. No GPU work on the sim thread. No audio mixing on the sim thread.
8. **Headless server is real.** Dedicated server must build and run with no window, GPU, or audio. Sim, physics, protocol, and world are tested that way.
9. **Mac daily, Windows honest.** Public APIs stay portable. No Metal-only (or Win-only) types leak across crate boundaries. We dogfood macOS; we do not write Mac-shaped architecture.
10. **Pack-shaped assets, one vendored pack.** The loader understands resource-pack layout and skin format. The repo vendors exactly one community pack (`assets/pack/`, VanillaTweaks defaults, unmodified, with its license/attribution file) so the game looks right out of the box; user-provided packs at runtime still override it. Do not commit third-party rips.

## Crate map

Keep this graph small. Names below are the intended workspace. Do not add crates without approval.

| Crate           | Owns                                                                                                                      | Must not own                                                            |
| --------------- | ------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| `engine-core`   | Time/tick, IDs, math re-exports (`glam`), small collections, shared error types                                           | GPU, audio, window, game rules, IO protocol                             |
| `engine-phys`   | AABB, swept/stepped voxel collision, Java-like character movement used by **both** server authority and client prediction | Rendering, net, items, crafting                                         |
| `engine-input`  | Our input snapshot/events, bindings                                                                                       | winit types, gameplay decisions                                         |
| `engine-render` | Scene submit API, meshing-to-GPU upload, frame graph, backends                                                            | World mutation, block rules, wgpu types in the public API               |
| `engine-audio`  | Mixer, positional voices, volume categories (master/music/players/weather)                                                | Gameplay, cpal types in the public API                                  |
| `world`         | Sections/chunks/palettes, entity store, spatial queries, chunk interest                                                   | GPU meshes as truth, protocol bytes, recipes                            |
| `protocol`      | Message enums, versioning, binary codec, client/server framing                                                            | wgpu, filesystems, game balance numbers beyond what the wire must carry |
| `game`          | OpenCraft rules once specced (blocks, items, crafting, interactions)                                                      | Renderer, OS, "just dump this draw call"                                |
| `server`        | Tick loop, authority, player sessions, interest, persistence hooks, embeddable + dedicated entry                          | Window, GPU, audio device                                               |
| `client`        | Window, render interpolations, local prediction, HUD/menus, pack load, optional embedded server                           | A second copy of sim rules                                              |

Binaries: `client`, `server`. The client may start an embedded server for SP/listen. That embed is a function call into the `server` crate, not a fork of its logic.

## Data and control flow

**Tick.** Server advances a discrete tick. Clients render every frame with a partial tick (`(now - last_tick) / 50ms`, clamped) so motion looks like Java, not like a 20 FPS slideshow.

**Voxels.** Paletted 16³ sections. The server owns the voxel truth. The client holds a replica for collision prediction and meshing. Meshing is a worker job from a chunk snapshot, not a steal of live server memory.

**Entities.** Generational entity ID + dense SoA component arrays (position, velocity, AABB, kind, …). Adding a component system that requires archetypes, queries-as-a-framework, or a crate is a rejected change unless this file is updated.

**Net.** Snapshot/interest for ~20 players. Local player: predict, replay, correct. Others: interpolate from server poses. Do not build lockstep. Do not trust the client.

**Coords.** Y-up, right-handed (+X east, +Z south). One `glam` stack everywhere. Do not introduce a second math crate or a render-only coordinate space without an explicit conversion type.

## Rendering, audio, UI

- Renderer consumes _submitted_ geometry/materials/camera from the client. It does not decide what a block is.
- Own bitmap/voxel-game UI. No egui (or other imgui) in shipped gameplay menus. Debug overlays are not an excuse to make the pause screen in egui.
- Own mixer on cpal: positional + categories. Sounds are pack-driven. The mixer is an engine, not `audio.play("foo.mp3")` sprinkled through game rules.

## Performance contract

Daily target on Apple Silicon: **uncapped render, locked 20 TPS, ~20 chunk render distance, 120 FPS, fans at or under ~25% on the test machine.** That is a design pressure, not a number to fake with vsync tricks. If a change needs a bigger abstraction to maybe go faster later, it is probably wrong. Measure the hot path (mesh, upload, draw, chunk IO, tick) before adding caches/layers.

## Platform and deps

**Allowed without asking:** Rust std, `winit`, `wgpu`, `cpal`, `glam`, `bytemuck`, `thiserror`, `anyhow` (binaries only), `parking_lot`, `rustc-hash`, `serde`, `serde_json` (resource packs / human config only), **one** binary codec used by `protocol` and saves, `png` / `jpeg-decoder` (resource-pack and skin images only), `lewton` (pack OGG/Vorbis only).

**Default deny.** Everything else (image codecs, extra ECS, physics engines, UI kits, async runtimes, networking frameworks, logging stacks, `bevy-*`) needs user approval and an allowlist edit here.

Hot path is not JSON. Pack files may be JSON because pack formats are. Protocol and world saves are binary.

## Rust patterns

- **Edition:** workspace Cargo, one edition, stable toolchain. Portable `cfg` for OS, not scattered `#[cfg(target_os = "macos")]` business logic.
- **Errors:** `Result`/`Option` in engine and game libraries. No `unwrap`/`expect` unless the comment names the invariant and why a bug-panic is correct. IO/net/asset boundaries return errors; they do not crash the sim thread as a vibe.
- **Unsafe:** only in platform/backend FFI or tiny proven hot kernels, wrapped, documented with a safety comment. No `unsafe` in `game` rules.
- **No inline imports.** Imports at the top of the module.
- **Exhaustive matches.** Switches on enums/unions use a `never` default so new variants fail compile.
- **No Bevy, no general-purpose engine.** We are the engine.
- **Data first.** Layouts and tick flow beat clever traits. Do not build an abstraction for a single backend.

## Testing

- Server/sim/physics/protocol/world: headless unit and integration tests. This is the real suite.
- Client prediction should be testable against the same `engine-phys` + `game` code the server uses.
- Render/audio: later smoke, not a wall of snapshot PNGs on day one.
- Do not skip a sim test because "we'll see it in game."

## Banned moves

- Outside protocol compatibility, foreign world loaders, or "temporary" third-party net packets.
- Committing third-party rips. The one vendored community pack named in invariant 10 is the only exception.
- Decompiling or pasting third-party source.
- Simulating on the render thread, or issuing GPU work from the server tick.
- Leaking `wgpu`/`winit`/`cpal` types into `game`, `world`, or `server`.
- A second character controller "just for the client."
- A second world representation "just for rendering" that can disagree with the replica (a mesh cache is fine; a shadow voxel store with different rules is not).
- New deps, crates, threads, or ECS frameworks without updating this file.
- Feature-spec content (item IDs, recipes, biomes) living only in chat — that goes in the forthcoming game design spec, then into `game`.

## Changing this file

Propose a diff. Explain the collision and the replacement invariant. Do not weaken a rule with "we'll be careful." The user approves, then the file changes, then code may follow.
