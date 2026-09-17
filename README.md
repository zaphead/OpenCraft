# OpenCraft

From-scratch Minecraft-shaped voxel game in Rust: our engines, our protocol, Java-like 20-tick feel. Native Mac now, Windows later.

Boot straight into a new infinite seeded world — no title screen. Walk, sprint, sneak, jump, punch dirt, craft planks and tools, place chests and crafting tables, die to falls and pick your drops back up.

> `AGENTS.md` is law in this repo. Leftover files are not authority. No Mojang protocol, no Anvil worlds, no committed Mojang assets.

## Quickstart

```bash
cargo run -p client -- --seed 1234 --pack /path/to/pack
cargo run -p server -- --seed 1234
```

Flags / env (both binaries):

- `--seed <i64>` or `OPENCRAFT_SEED` — random from system time if omitted
- Client only: `--pack <path>` or `OPENCRAFT_PACK` — Java resource-pack layout + vanilla skin format. Missing pack = missing-texture fallback, never a crash.

Requires stable Rust, a window/GPU for the client. The dedicated server is headless: no window, GPU, or audio.

## How it plays

- **Boot-to-world:** client spawns an embedded server in-process (same `server` crate as dedicated, not a fork). Pause shows the seed; quit discards the world with confirm.
- **Body:** WASD walk, mouse look, Space jump, Ctrl sprint, Shift sneak, E inventory, F5 first/third person, F2 XYZ-only debug, Esc pause. 1–9 + wheel select hotbar. Left-click dig, right-click place/use.
- **Survival-lite:** hearts + fall damage, instant respawn at spawn on death with inventory dropped as pickupable item entities. No hunger, mobs, armor, or death screen.
- **Blocks (core slice):** grass, dirt, stone, cobble, log, planks, crafting table, chest. Logs→planks→sticks→table/chest/wooden tools, sticks+cobble→stone tools.
- **UI:** own bitmap UI (`Panel`, `Text`, `Button`, `Slot`, `Crosshair` → `HudLayer`, `Hotbar`, `HeartRow`, `DebugMeter`, `ContainerScreen`, `PauseMenu`). No egui/imgui in gameplay.

## Architecture

Two binaries, one sim. Server is god at 20 TPS / 50 ms ticks; client predicts the local player, interpolates others, renders every frame with partial-tick.

```text
              dedicated server bin (headless)
                         |
                      protocol
                         v
winit --> client bin --> cpal
          predict + interp
          can EMBED server
          |      |      |
       render  input  embedded server
       (wgpu)  (ours) (same server crate)
```

Invariants: custom protocol + custom saves, Java-physics body clock, paletted 16³ sections, generational-ID SoA entities, `glam` Y-up (+X east, +Z south), wgpu/cpal/winit stay behind engine APIs.

## Crates

| Crate | Owns |
| --- | --- |
| `engine-core` | tick/time, IDs, `glam` re-exports, errors |
| `engine-phys` | AABB, voxel collision, shared character movement (server + prediction) |
| `engine-input` | input snapshot/events, bindings |
| `engine-render` | submit API, meshing upload, frame graph (wgpu private) |
| `engine-audio` | mixer, positional voices, master/music/players/weather (cpal private) |
| `world` | sections/chunks/palettes, entity store, spatial queries |
| `protocol` | message enums, versioning, binary codec |
| `game` | blocks, items, crafting, mining, survival, worldgen |
| `server` | tick loop, authority, sessions, interest, embeddable + dedicated entry |
| `client` | window, prediction/interp, HUD/menus, pack load, embedded server |

Dependency direction: `client` → render/audio/input/protocol/game/world/server; `server` → protocol/game/world/phys/core; `game` → world/protocol/phys/core. `server` never touches render/audio/window.

## Performance / deps / rules (short)

- Target: uncapped render, locked 20 TPS, ~20-chunk view distance, 120 FPS on Apple Silicon. Measure mesh/upload/draw/chunk-IO/tick before caching.
- Allowed deps: std, `winit`, `wgpu`, `cpal`, `glam`, `bytemuck`, `thiserror`, `anyhow` (bins), `parking_lot`, `rustc-hash`, `serde`/`serde_json` (packs/config), one binary codec, `png`/`jpeg-decoder` (pack/skin images), `lewton` (pack OGG). Everything else needs an `AGENTS.md` allowlist edit + approval.
- Rust: workspace Cargo, stable, `Result`/`Option` in libs, `unsafe` only in backend FFI/hot kernels, no inline imports, exhaustive matches, no Bevy/general engine.
- Tests: headless unit + integration for sim/phys/protocol/world; prediction tested against the same phys/game code as the server.
