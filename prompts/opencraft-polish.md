# OpenCraft polish: pack art, true faces, tight body, leaves, day and night

A single-mission prompt: make the shipped OpenCraft core look right and run right — pack-driven block/UI art, solid faces with correct winding and orientation, sticky/slidey movement fixed, 120 FPS with cool fans, leafy trees, and a 2-minute day-night cycle with crisp shadows that cost almost nothing. Critics grade the diff. The user clicks the named views. Done is the picture below, the UI spec, plus those gates — not the first thing that kinda works.

Task ID opencraft-polish
Scope Polish + small content on the shipped core (leaves, light, sky)
User clicks 7 named views

## Implementation Environment

|                                    |                                                                                                                                                                                                                                                |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Workspace                          | `/Users/spencerterry/Business/Sharp Stack/Software/Fun-Projects/OpenCraft`                                                                                                                                                                      |
| Starting point                     | Existing repo. The shipped OpenCraft core (boot-to-world client, server-god sim, bitmap UI kit) is authority. Match it; do not rewrite it.                                                                                                      |
| Closest pattern                    | The current tree itself: `crates/client` world canvas + HUD + `ContainerScreen` overlays, `engine-render` submit API, `game::face_offset` / `layout` / `SlotMap` canonical helpers.                                                              |
| Harness                            | Cursor. Fresh-context critic subagents **required**. They receive the mission goal, the UI spec, the diff, the rubric, and the inherited rules — not a builder summary, not a fake evidence folder. No self-grade. A failed spawn is a failed cycle: retry. |
| Checkpoints                        | Git. Do not add DIRECTION.md, TASK_STATE.md, evidence/, or other mission ceremony files to the repo.                                                                                                                                           |
| Stop signs (override the gauntlet) | Repo rules win. Stop and AskQuestion before adding a new primitive, recipe, or backend pattern. Implement the UI spec's kit exports; do not remap job-class words or freelance a different look. Stop and ask the user to click the §8 views before declaring done. Do not invent product requirements. |

## 01 / TASK

You are the lead builder and the final quality owner. Perform the actual work. Do not answer with only a plan, tutorial, or sample.

Write a plan if you need one. This brief is not that plan. It is what done looks like.

Git is enough to resume: reread this brief and continue from the working tree.

## Inherited Rules — hard gates

- This file is binding until the user approves a change to it; if a user request fights it, say what collides, what it costs, and what file change is required, then wait for approval; agents may argue a rule is wrong, never ignore one; do not over-specify downstream content or invent crates, deps, patterns, or backends "just this once" (AGENTS.md Agent law).
- New crates, new allowed deps, new threads, new backends, or a new data model require an AGENTS.md edit **and** user approval; prefer deleting and simplifying over adding (AGENTS.md Agent law).
- Two binaries, one sim: `server` is always the authority; singleplayer embeds that same server in-process; no second implementation of game rules (AGENTS.md System shape).
- Allowed depends-on: client → engine-render, engine-audio, engine-input, protocol, game, world, server; server → protocol, game, world, engine-phys, engine-core; game → world, protocol, engine-phys, engine-core; world → engine-phys, engine-core; engine-render → engine-core (wgpu private); engine-audio → engine-core (cpal private); engine-input → engine-core; engine-phys → engine-core; protocol → engine-core; engine-core → std + allowed math/infra only (AGENTS.md System shape).
- `server` must never depend on `engine-render`, `engine-audio`, `winit`, `wgpu`, or `cpal`; game rules must never depend on GPU/audio/window types (AGENTS.md System shape).
- Spiritual clone, not a wrapper: custom protocol, custom world saves; no Mojang net protocol, no Anvil/vanilla worlds (AGENTS.md invariant 1).
- Server is god: 20 TPS, 50 ms ticks; client predicts local player, interpolates others, applies corrections; clients do not authoritatively place/break/move (AGENTS.md invariant 2).
- Java Edition body clock from public knowledge; feel-test after; never decompile Mojang code (AGENTS.md invariant 3).
- wgpu is a rental: `engine-render` exposes our command/resource API; `game`, `world`, `server`, and client gameplay never see wgpu types (AGENTS.md invariant 4).
- cpal/winit are rentals too; platform types stay in the owning crate/module (AGENTS.md invariant 5).
- Chunks are not entities: 16×16×16 paletted sections; entities are generational IDs + SoA; no ECS crates, no chunks-as-entities (AGENTS.md invariant 6).
- Thread graph is fixed: render thread (uncapped), sim thread (20 TPS), audio thread, chunk-gen + mesh worker pool; no sim on render, no GPU on sim, no audio mixing on sim (AGENTS.md invariant 7).
- Headless dedicated server must build and run with no window, GPU, or audio (AGENTS.md invariant 8).
- Public APIs stay portable; no Metal-only (or Win-only) types across crate boundaries (AGENTS.md invariant 9).
- Pack-shaped assets, one vendored pack: Java resource-pack layout and vanilla skin format; exactly one community pack (`assets/pack/`, VanillaTweaks defaults, unmodified, with license/attribution) ships in git and user packs override it at runtime; raw Mojang/Microsoft rips stay banned (AGENTS.md invariant 10).
- Crate map is closed: `engine-core`, `engine-phys`, `engine-input`, `engine-render`, `engine-audio`, `world`, `protocol`, `game`, `server`, `client`; binaries `client`, `server`; embed is a function call into `server`, not a fork (AGENTS.md Crate map).
- Tick: server advances a discrete tick; clients render every frame with clamped partial tick `(now - last_tick) / 50ms` (AGENTS.md Data and control flow).
- Voxels: paletted 16³ sections, server owns truth, client holds a replica; meshing is a worker job from a chunk snapshot (AGENTS.md Data and control flow).
- Entities: generational ID + dense SoA; no archetype/query framework without an AGENTS.md update (AGENTS.md Data and control flow).
- Net: snapshot/interest, predict/replay/correct local player, interpolate others; no lockstep, no trusted client (AGENTS.md Data and control flow).
- Coords: Y-up, right-handed, Java-shaped (+X east, +Z south); one `glam` stack; no second math crate or render-only space without an explicit conversion type (AGENTS.md Data and control flow).
- Renderer consumes submitted geometry/materials/camera; it does not decide what a block is (AGENTS.md Rendering, audio, UI).
- Own bitmap/voxel-game UI; no egui (or other imgui) in shipped gameplay menus (AGENTS.md Rendering, audio, UI).
- Own mixer on cpal with categories (master/music/players/weather); sounds are pack-driven, never `audio.play` sprinkled through rules (AGENTS.md Rendering, audio, UI).
- Performance contract: uncapped render, locked 20 TPS, ~20 chunk view distance, 120 FPS, fans at or under ~25% on Apple Silicon; measure the hot path (mesh, upload, draw, chunk IO, tick) before adding caches/layers (AGENTS.md Performance contract).
- Allowed without asking: std, `winit`, `wgpu`, `cpal`, `glam`, `bytemuck`, `thiserror`, `anyhow` (binaries only), `parking_lot`, `rustc-hash`, `serde`, `serde_json` (packs/human config only), one binary codec for protocol/saves, `png`/`jpeg-decoder` (pack/skin images), `lewton` (pack OGG); everything else needs approval + allowlist edit (AGENTS.md Platform and deps).
- Hot path is not JSON; protocol and saves are binary (AGENTS.md Platform and deps).
- One workspace Cargo, one edition, stable toolchain; portable `cfg`, no scattered OS-gated business logic (AGENTS.md Rust patterns).
- `Result`/`Option` in libraries; no `unwrap`/`expect` unless the comment names the invariant; IO/net/asset boundaries return errors, never crash the sim thread as a vibe (AGENTS.md Rust patterns).
- `unsafe` only in platform/backend FFI or tiny proven hot kernels, wrapped with a safety comment; no `unsafe` in `game` (AGENTS.md Rust patterns).
- No inline imports; exhaustive matches with a `never` default so new variants fail compile; no Bevy; data first, no single-backend abstractions (AGENTS.md Rust patterns).
- Server/sim/physics/protocol/world get headless tests; prediction tests against the same `engine-phys` + `game` code; never skip a sim test because "we'll see it in game" (AGENTS.md Testing).
- Banned: vanilla protocol/Anvil, committing Mojang assets outside the one vendored pack, decompiling Mojang, sim on render thread, GPU work from the server tick, `wgpu`/`winit`/`cpal` in `game`/`world`/`server`, a second character controller "just for the client", a second world representation "just for rendering" (a mesh cache is fine; a shadow voxel store with different rules is not), new deps/crates/threads/ECS without updating AGENTS.md, feature-spec content living only in chat (AGENTS.md Banned moves).

## Diff gauntlet

After a coherent change, spawn one fresh-context critic per lens. Loop **until the critics pass** — no mandatory cycle count.

build or change → spawn critics on the diff → fix from their findings → repeat **only the lenses that failed**

Each spawn receives **only** the mission goal, the UI spec, the diff (or equivalent: changed files), the rubric, and the inherited rules. Do not pass notes, a file tree tour, or your score. If a spawn fails, retry it — never grade that lens yourself. The builder does not score the rubric.

A critic that returned a full pass is done. Do not spawn it again. Next rounds spawn **only** the lenses that failed. Keep the passing scores. Re-run a passed lens only if that spawn itself error'd and never produced a verdict.

1. **Rules critic.** Inherited gates (including thermonuclear / code criteria if inlined) against the diff, not against claims.
2. **Pattern/reuse critic.** Double-builds, ignored existing solutions, parallel stacks, invented primitives/patterns.
3. **Coverage critic.** Every named part exists in the work. Built surfaces match the UI spec (topology, regions, **exact kit exports**, sibling keep/drop/adapt, states, wire) when a spec is present. A job class implemented with the wrong export fails. Missing sibling chrome that was `keep` fails. Matching a sibling's empty state passes. A unique stub, closed door, or one-direction finish fails.
4. **Technical critic.** Naming, organization, editability, wrong layer, file sprawl.

Repair systemic issues before isolated polish. If the score stays below the exit threshold and barely moves across two consecutive critic rounds, change structure, not decoration.

**User gate.** Critics cannot sign off the product. After a critic pass, ask the user to click every §8 view. Fix what they report. You are not done until critics pass **and** the user has tested those views.

Do not drive the browser if the repo forbids it. Do not manufacture screenshots or `evidence/` files as a substitute. This is a native game: the user plays the client on Mac.

## 100-Point Rubric

Scored by the critics from the diff.

| Dimension                                     | Points  | Gate 100%      |
| --------------------------------------------- | ------- | -------------- |
| Inherited rules (incl. code criteria)         | 30      | 30             |
| Pattern and reuse                             | 25      | 25             |
| Coverage (named parts real; UI spec; sibling-empty OK) | 25      | 25             |
| Technical quality                             | 20      | 20             |
| **Total**                                     | **100** | **exit = 100** |

**Exit gates — all mandatory**

- 100 / 100 overall from the critics
- 100% of available points in every dimension
- Zero critical failures
- User has clicked every §8 view and reported them acceptable
- No material critic regression across the last two rounds

## 1. Project Objective

A played-in OpenCraft core: the same boot-straight-into-world survival-lite, now with vanilla-pack art on blocks and UI, faces that never go see-through, blocks that sit straight, a body that never sticks or skates, cool-and-fast frames, leafy trees, and a 2-minute day with crisp shadows that cost almost nothing.

**Defining story.** You launch the client. The game already looks right: block art, UI chrome, and sounds come from the vendored VanillaTweaks pack, with your own pack path overriding it if you pass one. Dirt reads as dirt, the hotbar and panels wear pack art, every block face is solid from every angle, trees carry leaf canopies. You sprint, sneak, and staircase up a hill without sticking or sliding; the fans stay quiet. The sun crosses the sky in two minutes from dawn; every trunk, canopy, block edge, and your own body throws a sharp shadow that swings with the sun; night falls dark and the shadows sleep; morning comes back — and the world, HUD, and containers behave exactly like before, only prettier, truer, and faster.

**Avoid these interpretations and shortcuts**

- The vendored pack is VanillaTweaks defaults from https://vanillatweaks.net/picker/resource-packs/, committed unmodified under `assets/pack/` with its license/attribution file; raw Mojang jar rips stay banned per AGENTS.md
- "Fixing" culling by disabling backface culling globally or double-drawing every face as a perf tradeoff
- Blob shadows, whole-chunk tinting, or a flat "night overlay" passed off as cast shadows — a shadow has a caster and an edge you can point at
- Cascaded shadow maps, PCF blur passes, or any shadow technique whose cost shows up on the fan/frame budget
- A second character controller, second collision path, or second world representation for shadows/rendering
- A day-night cycle that changes tick rate, sim rules, or spawn behavior
- New threads, crates, or deps slipped in without the AGENTS.md edit + approval
- Treating a unique stub as done
- Filling empty sibling states just to look complete

## 2. Scope, Adjacency, and Circulation

The complete result includes:

1. **Pack art, blocks + UI + sounds** — The vendored VanillaTweaks pack (`assets/pack/`) drives block tiles, UI chrome (hotbar, slots, hearts, panels, buttons), and sounds out of the box; a user-provided pack path overrides it at runtime; missing sprite falls back to the current procedural look, never magenta, never a crash.
2. **True faces** — Every block face renders solid from every view direction; winding/front-face is consistent across the mesher; no see-through sides, no direction-dependent disappearance.
3. **Straight blocks** — Block UVs/orientation realigned so grass-top reads on top, side grain runs vertical, logs read as logs from all sides; stretched or rotated faces are gone.
4. **Tight body** — Sneak never sticks on/off; jumping/stepping up hills never wedges the player inside terrain; no ice-skating (ground friction/drag behaves Java-like); sprint/sneak/jump feel unchanged when healthy.
5. **Cool fast frames** — Back toward 120 FPS with fans at or under ~25% on Apple Silicon; the hot path (mesh, upload, draw, chunk IO, tick) is measured first, then fixed — no vsync tricks, no speculative caches.
6. **Leaves** — Trees generate leaf canopies; leaves are solid, breakable blocks that vanish on break with no drop, no decay, no new recipes.
7. **Crisp cheap shadows + 2-minute day** — A full cycle runs 2 minutes from dawn: sun crosses the sky, world brightness follows, night is dark, morning returns. Shadows are the star: hard-edged and easy to read (you can point at a trunk's stripe across the grass and trace it back to the trunk), every block, canopy, and entity casts, edges stay sharp with no blur shimmer, and the whole thing is nearly free — frames and fans must read the same with shadows on as the post-perf-fix build without them. The cheap path (heightfield-style occlusion derived from the chunk snapshot on a worker, baked into the existing mesh/vertex data) is the shape to beat; a rival voxel store, an extra render pass per light, or a shadow-map pipeline is not.

**Out**

- Mobs, hunger, armor, PvP, dimensions, redstone, villages, boats/rails, commands
- Title menu, world list, world saves, in-game pack menu
- Leaf decay, saplings, leaf drops, new tools, new recipes
- Weather, seasons, torch light, moon-phase lore
- Second-player join as a required click (architecture stays MP-shaped)
- Any new crate, dep, thread, or backend without the AGENTS.md edit + approval

**Relationships that must hold**

- The pack path changes art only: same blocks, same inventories, same saves-shape, same sim outcomes with the vendored pack, an override pack, or no pack at all.
- Leaves obey the same rules as every block: server-authoritative break, replica prediction, drops/pickup semantics per this brief (vanish, no drop).
- Day-night is presentation + cheap shadow darkening: tick rate, physics, damage, and spawn logic do not know what time it is.
- Shadows never invent geometry: every darkened face traces to a real caster in the same snapshot the mesh came from.
- The body fixes live in the one shared `engine-phys` path the server and prediction both call.
- Perf fixes shrink hot-path work; they never add a maintenance-heavy layer to maybe go faster later.

Match the closest pattern. Compact is allowed. A closed door that should be a real unique part is not.

## 3. Families and Minimum Content

- **Art:** every shipped block tile + Hotbar/Slot/HeartRow/Panel/Button chrome reads from the pack when present; procedural fallback otherwise; magenta fallback is gone.
- **Faces:** consistent winding for all six faces; orientation-correct UVs per face (top/side/bottom where the block has them); mesher culls only truly hidden faces.
- **Body:** sneak toggle, step-up/jump-onto-block, ground friction; no new movement modes.
- **Perf:** measured hot-path numbers before/after; frame time and fan behavior visibly better on the test machine.
- **Leaves:** one leaf block: solid, breakable with a short break time, vanishes, no drop, no inventory special-casing beyond a normal stackable block.
- **Sky:** sun disc + sky brightness + 2-minute cycle from dawn; night dark; HUD/containers unaffected except world light behind them.
- **Shadows:** hard-edged, sun-tracking, cast by all blocks/canopies/entities; readable at a glance (caster → edge → ground); no blur, no shimmer, no flicker between frames; cost invisible on the frame/fan budget.

## 4. Connection Logic

This mission lands on the shipped core without forking it: same binaries, same sim authority, same protocol shape (extended only if the sky/light state must cross the wire — prefer deriving it from tick + seed client-side), same HUD slot language, same physics entry point. A later mission (title/saves/multiplayer join) must not have to unpick anything added here.

## 5. Language

OpenCraft / Java-shaped player language: sprint, sneak, hotbar, inventory, crafting table, chest, seed, hearts, spawn, first person, third person, leaves, canopy, dawn, dusk, shadows. Engines: `client`, `server`, `world`, `protocol`, `game`. Do not say "scene," "prefab," "Bevy system," "shader graph," or "creative inventory."

## 6. Context

AGENTS.md froze engines, ticks, crates, deps, and net authority. The shipped core (boot-to-world, survival-lite, hotbar/inventory/crafting/chest, F2 XYZ, pause-with-seed) is the sibling to match. The user's screenshot shows the failure modes verbatim: magenta crafting panel (broken missing-texture fallback), stretched mis-oriented tree trunks, flat unreadable terrain art, F2 XYZ readout working.

## 7. Empty, error, edge

No web-app empty states. Missing pack or missing sprite: procedural fallback, still playable, log/stderr note, never a crash, never magenta. Bad pack path: same as missing. Night: dark but navigable, HUD fully legible. Leaves with no pack art: fallback leaf color, still solid and breakable. Quit-without-confirm still forbidden (world is unsaved). 0 HP still respawns at spawn with drops, day or night.

## 8. Views the user will click

1. **Pack art** — Launch: blocks, hotbar/panels, and sounds wear the vendored pack; remove `assets/pack` and the same world renders procedural, no magenta anywhere.
2. **True faces** — Walk around any hill/tree and look at every side from every angle: no see-through faces, no flickering disappearances.
3. **Straight blocks** — Grass tops on top, trunk grain vertical, planks/stone reading correctly up close.
4. **Tight body** — Sprint, sneak-toggle, and staircase-jump up a steep hill: no sticking, no wedging, no ice-skating.
5. **Cool frames** — Play several minutes: frames near the 120 class, fans quiet (≈25% or under), no mid-play hitches.
6. **Leafy trees** — Find trees: full canopies, punch leaves (they vanish, nothing drops), stand on a canopy.
7. **Day and night** — Watch two full minutes: dawn → short noon shadows → dusk → dark night → dawn; every shadow is sharp, traces to a visible caster, swings with the sun, and costs nothing you can hear or count.

Collectively cover every named part. Happy path alone is not enough. This list is for the user, not for an `evidence/` folder.

## UI spec

### Job
A player keeps playing the same survival-lite loop while the world looks true: art they recognize, faces that hold, light that moves. Success is the defining story with zero new UI to learn.

### Out
Not a settings/video menu. Not a brightness slider. Not a time control or time display. Not a pack picker. Not a new HUD element. Not a leaf recipe or block catalog. No-pack launch still works (procedural fallback); a pack path flag/env overrides the vendored pack.

### Topology
**Workspace** (play viewport + HUD) stays home. **Overlay** interrupts exactly as today. No new surfaces, no transitions to spec.

### Hierarchy
- Primary: the world in front of the crosshair (now correctly faced, oriented, lit, shadowed)
- Secondary: hotbar + hearts (now pack-backed)
- Tertiary: F2 XYZ; pause seed; sky as ambience, never a control

### Sibling
Closest shipped surface: the current game (`crates/client/src/ui`, `crates/client/src/app`, world canvas submit path)
| Chrome / control / state | Keep / drop / adapt | Why |
| --- | --- | --- |
| World canvas + Crosshair | keep | Aim and terrain stay the interaction |
| Hotbar, HeartRow, DebugMeter (XYZ) | keep | Same HUD, same positions |
| ContainerScreen (inventory/table/chest) | keep | Same slot language and layout |
| PauseMenu (seed, Back to Game, Quit+confirm) | keep | Same pause contract |
| Procedural atlas fallback | keep | Missing-pack playability |
| Magenta missing-texture panel | drop | The screenshot's failure mode; fallback must be procedural, never magenta |
| Block tiles + Slot/Button/Panel/heart art | adapt | Same kit exports, art sourced from pack sprites when present |
| Sky + world brightness + crisp shadows | adapt | World presentation gains sun cycle and hard-edged cast darkening; no new widget |

### Kit
- Primitives: `Panel`, `Text`, `Button`, `Slot`, `Crosshair` (exact, unchanged)
- Compositions: `HudLayer`, `Hotbar`, `HeartRow`, `DebugMeter`, `SlotGrid`, `ContainerScreen`, `PauseMenu` (exact, unchanged)
- Recipes consulted (do not import): none
- Gaps (AskQuestion, do not invent): none — every region below binds to an existing export

### Wire

Play (workspace):

```
┌ F2 DebugMeter (XYZ, off unless toggled)          ┐
│                                                   │
│        world canvas (sun, crisp shadows,          │
│         leaves, true faces, pack art)             │
│                      + Crosshair                  │
│                                                   │
│ HeartRow (pack art)        Hotbar (pack art)      │
└───────────────────────────────────────────────────┘
```

Overlays: unchanged `ContainerScreen` / `PauseMenu` wires from the shipped core; pack art backs the same slots and buttons.

### Regions

#### World canvas
- Job of this region: see and aim at a true, lit world
- Contents: terrain, leaf canopies, items, arm/player, particles, sun disc, sky brightness, hard-edged cast shadows
- Job class: `page` (workspace canvas)
- Kit: world submit through `engine-render` command API; sun/brightness/shadow ride the existing submit (uniforms/vertex darkening), not a UI widget
- Not: a settings-driven look; a second scene graph; a shadow-map pipeline with its own passes
- Grouping: world is the surface; HUD floats on it; sky and shadow are presentation of the same submit
- Named composition: none
- Inline: sun position, brightness, per-face/vertex shadow darkening, leaf quads

#### Crosshair
- Job of this region: aim
- Contents: centered crosshair while pointer-locked in play; hidden in overlays
- Job class: `badge`
- Kit: `Crosshair`
- Not: a mouse cursor while playing
- Grouping: alone at viewport center
- Named composition: none
- Inline: the mark

#### HeartRow
- Job of this region: show health
- Contents: 10 hearts, damage visible immediately, pack art when present
- Job class: `badge`
- Kit: `HeartRow`
- Not: a text HP number; hunger drumsticks
- Grouping: above/near hotbar start, not in pause
- Named composition: `HeartRow`
- Inline: heart pips

#### Hotbar
- Job of this region: choose what is in hand
- Contents: 9 slots, selected index, item counts, pack art when present
- Job class: `grid`
- Kit: `Hotbar` wrapping `Slot` × 9
- Not: a radial menu; a text item list
- Grouping: bottom-center in play; same row inside every `ContainerScreen`
- Named composition: `Hotbar`
- Inline: selection highlight

#### DebugMeter
- Job of this region: read XYZ
- Contents: player X/Y/Z when F2 on; nothing else
- Job class: `meta`
- Kit: `DebugMeter`
- Not: F3 dump, fps, biome, spawn, facing, targeted block
- Grouping: top-left, away from hotbar
- Named composition: `DebugMeter`
- Inline: the three numbers

#### ContainerScreen
- Job of this region: move items; craft if the screen has a grid
- Contents: same `SlotGrid`s as shipped; pack art backs the same slots
- Job class: `dialog`
- Kit: `ContainerScreen` + `SlotGrid` + `Slot` + `Panel`
- Not: egui window; creative catalog; armor column
- Grouping: craft/chest grids above player storage; hotbar bottom row; result with craft grid
- Named composition: `ContainerScreen`
- Inline: labels only to separate chest vs player; no extra tabs

#### PauseMenu
- Job of this region: stop play, read seed, resume or quit
- Contents: "Game paused", seed `meta`, Back to Game, Quit → "World is not saved" confirm
- Job class: `dialog`
- Kit: `PauseMenu` + `Panel` + `Text` + `Button`
- Not: settings; title menu; egui
- Grouping: seed with paused identity; Resume vs Quit apart
- Named composition: `PauseMenu`
- Inline: confirm is a second moment on Quit

### Layout
**Wide:** world is the canvas; HUD pinned to edges; container/pause panels centered. One overlay at a time.
**Narrow:** still a desktop game; HUD scales down; panels stay centered; no mobile nav.
**Touch:** not a touch product; click is the verb; pointer lock in play, unlocked in overlays.

### States
| State | What the user sees | Sibling match |
| --- | --- | --- |
| Empty | Empty `Slot` frames; world still generated | keep — slot chrome is the empty |
| Loading | New chunks appear as you walk; no page spinner | keep |
| Error | Missing pack → procedural art + stderr note; never magenta, never crash | adapt — magenta fallback dropped |
| Overflow | Stacks cap like Java; extras stay as entities | keep |
| Permission | N/A (local player) | drop |
| Destructive | Quit from pause → confirm "World is not saved" | keep |

### Feedback
No toasts. Break/place: particles (+ pack sound if present). Pickup: item leaves world, appears in inventory. Craft: ingredients consume, result appears. Damage: hearts empty, hurt feedback. Death: items drop, respawn at spawn. Dawn/dusk/night: silent, continuous, no banner.

### Anti-goals
egui/imgui. A brightness slider or clock widget. A pack browser. New HUD meters (fps, light level, coordinates beyond XYZ). Different hotbar art per container. Magenta anything. Flat-shaded "night mode" overlay that ignores terrain. Blob shadows or whole-chunk tinting. Shadow maps, blur passes, or extra per-light render passes. Double-drawn faces as a culling fix.

## 9. Critical Failures

- A named part is missing or only a label, and it is not an intentional sibling-empty
- Built UI contradicts the UI spec (wrong topology, extra surfaces, missing regions, wrong kit export, omitted sibling `keep` chrome, a look that fights the spec)
- Existing solutions ignored for a parallel stack
- A harvested hard repo rule is violated
- A new primitive/recipe/pattern shipped without the required stop-and-ask
- Declared done without the user clicking the §8 views
- Concealment (mocks, placeholders, fake evidence files)
- Mojang/Microsoft IP downloaded, fetched, or committed anywhere in git history, outside the one vendored community pack named in AGENTS.md invariant 10 (which ships unmodified with its license file)
- Backface culling disabled globally or faces double-drawn to hide a winding bug
- See-through faces from any angle, or stretched/rotated block faces
- A shadow without a traceable caster; soft/blurry shadow edges; shadow shimmer or flicker; measurable frame/fan cost from shadows
- Sticky sneak, terrain wedging, or ice-skating still reproducible
- Frames far from the 120 class or fans loud on the test machine without measured hot-path justification
- Leaves that drop items, decay, need recipes, or ignore server authority
- Shadows from a rival voxel store, or a day cycle that touches tick rate, physics, or spawns
- Hunger, mobs, or a settings menu sneaking in

## 10. Deliverables

The product in this repo. Nothing else. No mission README, evidence pack, or recovery markdown.
