# Living world

A single-mission prompt: build the OpenCraft world the player can walk — biomes, set-pieces, flora, weather, and the mobs and fantasy systems named below — on the shipped sim, not beside it. Critics grade the diff. The user clicks the named views. Done is the picture below, the UI spec, plus those gates — not the first thing that kinda works.

Task ID living-world
Scope The played world: biomes, life, set-pieces, atmosphere, and the fantasy systems named here
User clicks 8 named views

## Implementation Environment

|                                    |                                                                                                                                                                                                                                                |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Workspace                          | `/Users/spencerterry/Business/Sharp Stack/Software/Fun-Projects/OpenCraft`                                                                                                                                                                      |
| Starting point                     | Existing repo. The shipped core (boot-to-world, survival-lite, 2-minute day, pack art, crisp shadows, leafy trees, bitmap UI) is authority. The working tree may already hold unrelated render/input edits; leave them unless this mission truly owns that line. |
| Closest pattern                    | The current tree: `generate_chunk` / `surface_y` purity, `EntityKind` + SoA in `world`, one `tick_body` in `engine-phys`, `WindowKind` + `ContainerScreen`, `HudLayer` / `HeartRow`, `Particle` bursts, mixer categories, 64×64 skin-boxes, 16×16 atlas. |
| Harness                            | Cursor. The lead **must** spawn many feature subagents, one per named feature in the Swarm section, in parallel waves. Fresh-context critic subagents **required** after integration. Critics receive the mission goal, the UI spec, the diff, the rubric, and the inherited rules — not a builder summary, not a fake evidence folder. No self-grade. A failed spawn is a failed cycle: retry. |
| Checkpoints                        | Git. Do not add DIRECTION.md, TASK_STATE.md, evidence/, or other mission ceremony files to the repo.                                                                                                                                           |
| Stop signs (override the gauntlet) | Repo rules win. Stop and AskQuestion before adding a new primitive, recipe, crate, dep, thread, backend, or data model. Implement the UI spec's kit exports; do not remap job-class words or freelance a different look. One working tree only: feature subagents are mandatory, and they edit this repo in place. No per-feature branch, worktree, or second sim. Drive the headless suite and a live Mac client through the §8 views before asking the user to click them. Do not invent product requirements past this brief. |

## 01 / TASK

You are the lead builder and the final quality owner. Do not build this alone. Do not answer with only a plan, tutorial, or sample.

Write a plan if you need one. This brief is not that plan. It is what done looks like.

Git is enough to resume: reread this brief and continue from the working tree.

## Swarm — required

Spawn many subagents, one per feature below, and run as many at once as the tree allows. You integrate. They do not each own the repo.

Same checkout. Same branch. No worktree per feature, no best-of-n fork, no second copy of `game` / `world` / `server`. A feature that needs a new crate, dep, thread, or data model stops and asks; it does not invent one inside its slice.

You own the shared seams, and you land them before the wave that needs them: `EntityKind` and the SoA columns, `tick_body`, block and item ids, `generate_chunk` biome entry, `WindowKind` / `OpenKind`, the atlas slots, the protocol snapshot, `HudLayer`. Assign each subagent a disjoint write set. Only you edit a shared file. A subagent that must touch one sends you the behavior; you merge it. If two slices collide, you serialize them. You do not let them overwrite each other.

Each subagent gets this brief, its one feature, the seams you already landed, and the out-list. It does not get a tour of the repo or permission to redefine the feature.

Features, one subagent each:

- Aurochs
- Wild boars
- Mountain goats
- Rabbits and hares
- Deer stags
- Foxes
- Songbirds
- Crows
- River fish
- Pond frogs
- Glowbeetles and the companion lantern
- Honeybees and hive blocks
- Horses (tame, breed, ride, sprint stamina)
- Griffins (glide)
- Goblins
- Husk revenants
- Bog wraiths
- Frost wights
- Ash imps
- Spore hoppers
- Mimics
- Stone sentinels
- Treant elder
- Skywhales and the hunters that follow them
- The Wyrm
- Blood-moon surge
- Sunpetal meadow
- Old oakwood
- Birch grove
- Pine taiga
- Redwood highlands
- Dune sea and oasis
- Painted mesa
- Savanna and baobabs
- Jungle deep
- Willow swamp
- Mushroom isle
- Ashlands
- Frostfields and glacier
- Glowwood
- Haunted moor
- Coral shallows
- Sky isles
- Crystal grotto
- Fairy rings
- Ley monoliths and etch
- Sunken ruins
- Desert temples
- Watchtowers
- Goblin camps
- Beanstalk
- Natural arches, tavolo peaks, canyon rifts, craters
- Hot springs
- Geysers
- Thorn walls and bramble mazes
- Fallen logs, erratic boulders, ossified ribs
- Glowworm cenotes
- Rainbow bridge
- Biome flora (flowers, tuft grass, reeds, lily pads, thornbushes)
- Starblooms
- Sunpetals
- Cave glowvines and crystal buds
- Autumn canopies and falling leaves
- Fireflies, pollen, spores
- Ashfall, snowfall, jungle rain
- Aurora, shooting stars, blood-moon tint
- Wind sway
- Biome fog
- Biome music and mob calls
- Staves and spell projectiles
- Brewing and potions
- Sage traders
- Advancement list
- Keys and sealed vaults
- Dungeon spawners
- Boss bars
- Seasons (tint and snowfall only)

Waves are fine: seams first, then every feature whose seam exists, in parallel. A feature is not done because its subagent said so. You read the diff, reject a stub, and send that feature back.

Before you ask the user to click anything, the headless sim/phys/protocol/world tests for this mission are green, and you have driven a live client on Mac through every §8 view far enough to see the behavior. Come back only after that. The user click is the last gate, not the first look.

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
- Banned: vanilla protocol/Anvil, committing Mojang assets outside the one vendored community pack, decompiling Mojang, sim on render thread, GPU work from the server tick, `wgpu`/`winit`/`cpal` in `game`/`world`/`server`, a second character controller "just for the client", a second world representation "just for rendering" (a mesh cache is fine; a shadow voxel store with different rules is not), new deps/crates/threads/ECS without updating AGENTS.md, feature-spec content living only in chat (AGENTS.md Banned moves).

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

**User gate.** Critics cannot sign off the product. After a critic pass, and after you have already driven the live client through every §8 view, ask the user to click every §8 view. Fix what they report. You are not done until critics pass **and** the user has tested those views.

Do not drive a browser. Do not manufacture screenshots or `evidence/` files as a substitute. This is a native game: you play the client on Mac, then the user does.

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
- You have driven a live client through every §8 view, and the user has then clicked them and reported them acceptable
- No material critic regression across the last two rounds

## 1. Project Objective

A seeded OpenCraft world that is already alive when you boot: biomes you can walk between, mobs that behave, set-pieces with loot and teeth, weather and sky that belong to the place, and a short fantasy loop (etch, brew, ride, trade, a boss) on the same server that already runs survival-lite.

**Defining story.** You launch into a sunpetal meadow at dawn. Flowers track the sun, deer bolt as a herd, an aurochs drops hide and beef, songbirds call from somewhere you can point at. You walk into oakwood at dusk: fireflies, a fox steals the beef you dropped, a treant roots you and its bar shows over the hotbar. Night brings goblins from a camp, and every fourth night the sky goes blood-red and even the cattle want you dead. Farther out you find oasis water and husks, a mesa, baobabs and a griffin you can glide off a highland, swamp wraiths, a mushroom isle that pops, ashlands lava and the Wyrm, frostfields under an aurora, a glowwood where you catch a beetle and carry it as a lantern. You etch a tool at a humming monolith, brew a drink, tame a horse and sprint it until it tires, buy a key from a sage, open a vault, and read what you've done from the pause menu. A dedicated server and a second client see the same mobs you do. The old loop (dig, craft, chests, fall damage, pause seed, 2-minute day, shadows, pack art) still works.

**Avoid these interpretations and shortcuts**

- A second world, dimension, or portal, including the feywild sky-realm. That needs an AGENTS.md data-model edit and user approval; this mission does not ship it.
- A second character controller for horses, griffins, swim, or glide. One `tick_body` learns a mounted flag and a fluid answer.
- An ECS, behavior-tree crate, AI framework, or fluid cellular automaton. Mobs are more `EntityKind` rows. Water and lava are still blocks.
- Horse armor, hunger, boats, title menu, world saves, commands, redstone, villages.
- Mojang/Microsoft textures, sounds, or skins committed or fetched. Procedural art and original clips, pack files only when they are already in the vendored pack or a user pack.
- A mob that is only a renamed player skin with no behavior. A biome that never generates. A set-piece that is a comment.
- Day length or tick rate changed. The shipped 2-minute day stays.
- Treating a unique stub as done.
- Filling empty sibling states just to look complete.

## 2. Scope, Adjacency, and Circulation

The complete result includes:

1. **Biomes you can walk** — Spawn is always a sunpetal meadow. The other biomes generate from the seed within a few minutes' walk or a cave/sky trip, each with the life and set-pieces tied to it below. Same seed, same world.
2. **Farmable mobs** — Aurochs, wild boars, mountain goats, rabbits and hares, deer stags, foxes, songbirds and crows, river fish and pond frogs, glowbeetles, honeybees and hive blocks, horses, griffins. Behaviors in §3 are real on the sim.
3. **Hostile mobs and bosses** — Goblins, husk revenants, bog wraiths, frost wights, ash imps, spore hoppers, mimics, stone sentinels, treant elder, skywhales with hunters, the Wyrm, blood-moon surge.
4. **Set-pieces** — Fairy rings, ley monoliths, sunken ruins, desert temples, watchtowers, goblin camps, one beanstalk, natural arches, tavolo peaks, canyon rifts, craters, hot springs, geysers, thorn walls, bramble mazes, fallen logs, erratic boulders, ossified ribs, glowworm cenotes, a rainbow bridge.
5. **Flora** — Biome-dyed flowers, tuft grass, reeds, lily pads, thornbushes, starblooms, sunpetals, cave glowvines, crystal buds, autumn-tint canopies with falling leaves. Birch, pine, redwood, baobab, willow, and giant mushroom trunks are real block variants.
6. **Atmosphere** — Fireflies, pollen, spores, ashfall, snowfall, jungle rain, aurora, shooting stars, blood-moon tint, leaf/grass wind sway, biome fog, biome music, positional mob calls. Seasons are tint plus snowfall, presentation only.
7. **Fantasy loop** — Rune etch at monoliths, staves and projectiles, brewing, taming and breeding, companion lanterns, sage traders, an advancement list, keys and sealed vaults, dungeon spawners, boss bars.

**Out**

- Feywild portal or any second world model.
- Horse armor.
- Flowing-fluid simulation, buckets, finite water, bubble columns.
- Hunger bar, boats, rails, title menu, world saves, commands, redstone, villages, PvP rules.
- New crates, deps, threads, backends, or an AGENTS.md edit slipped in without stopping to ask.
- egui or any new UI kit.

**Relationships that must hold**

- The server authors every mob, drop, etch, brew, trade, tame, vault, spawner, and boss. The client predicts only the local body and interpolates everyone else.
- A dedicated server with no GPU shows the same world a client embeds. Interest still limits who you hear and see.
- Horses and griffins move the one body. Sneak dismounts. No new keybinds.
- Water and lava are non-solid blocks the mesher and the body already understand. Lava hurts. Hot-spring water heals while you stand in it. Geysers add upward velocity; the existing fall damage still applies when you land.
- Time of day is derived from the tick. Night spawns, starblooms, sunpetals, blood moon, and the rainbow syzygy read that clock. Tick rate stays 20. Seasons do not change spawns or recipes.
- New tiles use free slots on the existing 256-tile atlas (tiles 0–20 are already taken). Missing art is the procedural fallback, never magenta, never a crash. Mobs draw as 64×64 skin-boxes the way the player does.
- Sounds go through the mixer categories that already exist. A missing clip is silence.

Match the closest pattern. Compact is allowed. A closed door that should be a real unique part is not.

## 3. Families and Minimum Content

- **Clock:** day stays 2 minutes. Blood moon is every 4th night, dawn to dusk-of-night only, sky tinted, and every mob that spawns that night is hostile (farm animals included). Seasons are 4 days each, cycling tint; the cold season adds snowfall particles everywhere and does not change blocks or spawns. Rainbow syzygy is the dawn of every 4th day and lasts until the next dusk. Starblooms are open only while the world is in night brightness. Sunpetal heads track the sun across the day.
- **Meadow and woods:** sunpetal meadow (rolling hills, flower seas, sunpetals, aurochs, rabbits, deer herds that bolt together, pollen by day, songbirds). Old oakwood (dark canopy, treant elder, fireflies at dusk, foxes). Birch grove and pine taiga (new trunks, autumn tint, falling leaves, hares). Redwood highlands (colossus trunks with space inside you can stand in, goats on cliffs, autumn tint).
- **Dry:** dune sea (husks by day, oasis of still water), painted mesa (banded clay, a ruin chest with a gold-colored ingot item), savanna (flat-topped baobabs, griffins).
- **Wet:** jungle deep (plants to 3 blocks, parrots, rain while you are in it), willow swamp (hanging vines, bog wraiths, sinkholes), mushroom isle (giant mushrooms, spore hoppers, purple rain, spore particles), coral shallows (still water, coral blocks, fish), river and pond water with fish in it and frogs on the bank.
- **Harsh and strange:** ashlands (basalt, lava pools, ashfall, ash imps, one Wyrm in a lava arena), frostfields and glacier (aurora at night, frost wights, snowfall, packed-ice caves, snow-white hares), glowwood (night-glowing flora, wisps, catchable glowbeetles), haunted moor (grey grass, standing stones, graveyards, thorn walls). Crystal grotto is an underground biome of pickable crystal buds. Sky isles float, with waterfalls of still-water blocks pouring off the edge into air.
- **Farm mobs:** Aurochs drop hide and beef. Boars forage in forests and aggro when hit. Goats spawn on cliffs, can jump 2 blocks, and play a positional call when you get close. Rabbits match the biome; frostfields rabbits are white. Deer in a herd flee together when one is scared. Foxes are nocturnal and pick up a nearby dropped item entity. Songbirds, crows, and parrots are ambient flyers with positional calls. Fish swim in water blocks. Frogs sit at pond edges and hop. Glowbeetles wander glowwood; right-click catches one into a companion lantern. Bees visit flowers and, over ticks, place a new flower on an adjacent grass block; breaking a hive angers them and they sting. Horses right-click to mount, sprint drains a stamina pool that refills off-sprint, sneak dismounts. Griffins are rare on savanna and highlands; mounted, the body glides (forward and down, no hover) and sneak dismounts.
- **Hostile mobs:** Goblins raid at night from camps and watchtowers, with a positional squeak. Husks rise in the dune sea by day. Bog wraiths are slow and take more hits. Frost wights ambush in frostfields. Ash imps take no damage from lava. Spore hoppers on the mushroom isle explode for damage and spore particles in a small radius and clear flora in that puff, not stone. Mimics look like chests until opened, then bite and are a mob. Stone sentinels at ruins stay still until a ruin chest or vault is looted, then wake. Treant elder roots the player (velocity held at zero for a short time) and shows a boss bar. Skywhales are peaceful and huge; goblins spawn trailing them. The Wyrm is one boss in the ashlands lava arena, has a boss bar, and drops a wyrm scale on death.
- **Set-pieces, deterministic from the seed:** Fairy ring (flower circle, one king mushroom, wisps at midnight). Ley monolith (etch site, positional hum on the Music category while you stand near). Sunken ruin and desert temple (loot chest, sentinel, a spawner). Watchtower and goblin camp. One beanstalk per world that climbs to a sky isle. Arches, tavolo peaks, canyon rifts, craters. Hot springs (steam particles, heal while soaking). Geysers (launch, then fall damage). Thorn walls and bramble mazes that hurt on touch, moor borders. Fallen logs, erratic boulders, ossified ribs as litter. Glowworm cenotes (sinkhole, glowvines). Rainbow bridge of colored blocks at syzygy dawn, walkable onto a sky isle. Seed `1234` places meadow at spawn and puts each of these within a few minutes' walk, a short cave, or the beanstalk, so the views are reachable without a debug command.
- **Flora blocks:** flowers (dyed by biome), tuft grass, reeds, lily pads on water, thornbushes (damage), starblooms, sunpetals, glowvines, crystal buds. Vines and buds are placeable light: they render emissive. Lily pads, flowers, grass, and reeds are not solid. Autumn canopies in taiga and highlands use a tint, plus falling-leaf particles. Wind sway is one vertex wobble on leaves, grass, and flowers.
- **Drops and foods:** beef and honeycomb each restore one heart when eaten (right-click). No hunger. Hide is the sage's trade good. Honeycomb drops from a hive. Glowcap drops from isle mushrooms. Nether-ash drops from ashlands basalt. Starbloom drops from the flower at night.
- **Etch:** right-click a monolith. One tool slot. Etch once: a wood tool breaks as fast as stone; a stone tool breaks one step faster. No table block, no XP.
- **Staff:** craft sticks plus a glowcap. Right-click fires a projectile entity with velocity that damages a hostile and then despawns.
- **Brew:** a brew block you place and right-click. Three recipes only: nether-ash + glowcap = fire resistance (lava does not hurt for a short time); glowcap + starbloom = night glow (your brightness lifts); nether-ash + starbloom = swiftness (a short speed bump on the one body). All three together = a heal that fills your hearts.
- **Tame and breed:** feed beef to aurochs and horses, a flower to rabbits. Hearts particles, a baby that grows into the adult. Horses you tamed are the ones you can mount.
- **Sage:** a passive mob. Right-click trades: 8 hide → staff, 4 beef → key, 1 honeycomb → glowcap.
- **Advancements:** a list you open from pause. Entries start locked and check off when the sim has seen them: wake in the meadow, fell a tree, take a hide, ride, etch a tool, drink a potion, see a blood moon, strike the Wyrm. Children sit indented under those lines. No separate skill graph.
- **Key and vault:** the key opens one sealed vault in a sunken ruin. Without it the vault does not open. The key is consumed. After that the vault is an ordinary chest. Spawner blocks in ruins and temples emit the local hostile while a player is near.
- **Art and audio:** pack tile if the pack has it, else procedural. Per-mob procedural 64×64 skin, pack skin file overrides if present. Biome music on Music, weather on Weather, mob calls positional on Players. Original clips or pack OGG only.

## 4. Connection Logic

This lands on the shipped core. Boot is still straight into the world. Dig, craft, chests, fall damage, death drops, pause seed, F2 XYZ, the 2-minute sun, and crisp shadows stay. Mobs, fluid blocks, and opened vaults are part of the same snapshot a dedicated server would send. Etch, brew, trade, and vault are windows in the same family as the chest. The body you already predict is the body that swims, rides, and glides. A later save/title mission must not have to unpick a second sim.

## 5. Language

OpenCraft player language: sprint, sneak, hotbar, inventory, hearts, seed, spawn, dawn, dusk, biome names as in §3, hide, beef, hive, monolith, etch, brew, staff, vault, key, boss. Engines stay `client`, `server`, `world`, `protocol`, `game`. Do not say "scene," "prefab," "Bevy system," "shader graph," "dimension," or "creative inventory."

## 6. Context

AGENTS.md froze engines, ticks, crates, deps, and net authority. Shipped today: blocks grass through leaves, items through stone tools, windows Inventory / Crafting Table / Chest, entities Player and Item, one body with sneak and sprint, a 256-tile atlas with tiles 0–20 used, a 64×64 skin, particles, a mixer with Master / Music / Players / Weather, and a 2-minute day with sun direction and brightness uniforms. There is no fluid, no mob, no biome, and no second world. README still says no mobs; this mission is the one that adds them. The vendored pack is VanillaTweaks, unmodified. Do not replace it.

## 7. Empty, error, edge

No web empty states. Missing pack sprite or skin: procedural, playable, stderr note, never magenta, never a crash. Missing sound: silence. No boss in range: no boss bar. Vault without a key: it does not open, no error panel. Advancement list on a new world: every line visible and unchecked. Empty slots stay empty slot chrome. 0 HP still drops your inventory and respawns at the meadow spawn. Quit still confirms the world is not saved. Blood moon ends at dawn and the cattle are calm again. A griffin with no cliff still glides down, it does not hover. Stamina at empty stops the sprint until it refills. Bees with no grass nearby do not invent a flower in stone.

## 8. Views the user will click

Seed `1234` unless a view says otherwise. Collectively these cover every named part.

1. **Meadow morning** — Boot at dawn in a sunpetal meadow. Flowers and sunpetals track the sun. Pollen drifts. Deer bolt as a herd. Rabbits are meadow-tinted. An aurochs dies into hide and beef; eat the beef and a heart returns. Songbirds call from a direction you can turn toward. Birch and pine stand a short walk off, with autumn tint and falling leaves.
2. **Woods at dusk** — Oakwood canopy, fireflies, a fox picks up a dropped item, a treant roots you and a boss bar shows. A fairy ring grows a king mushroom, and wisps show at midnight. Starblooms are shut by day and open at night.
3. **High country and sky** — Stand inside a redwood. A goat jumps two blocks and calls. Mount a griffin and glide down, sneak to get off. Climb the one beanstalk to a sky isle whose waterfall pours off the edge. At syzygy dawn, walk the rainbow bridge onto an isle. See an arch, a tavolo peak, a canyon, a crater.
4. **Dry country** — Dune sea, oasis water, husks by day. Painted mesa and a chest with a gold ingot. Savanna baobabs. A desert temple: loot the chest, the sentinel wakes. A mimic chest bites.
5. **Wet country** — Swamp vines, a wraith, a sinkhole. Jungle undergrowth, parrots, rain. Mushroom isle, a spore hopper pops, purple rain and spores. Fish in the river, frogs on the bank, lily pads. Coral shallows. Reeds and tuft grass. A hot spring heals while you soak. A geyser throws you and the landing hurts.
6. **Harsh country** — Ashlands basalt, lava that hurts, ashfall, an imp standing in lava unhurt, the Wyrm with a boss bar and a scale drop. Frostfields: aurora, snowfall, a wight, packed ice, a white hare. Haunted moor: grey grass, standing stones, a graveyard, thorns that hurt, a bramble maze. A goblin camp and a watchtower squeak at night. Wait to the 4th night: the sky tints and the aurochs aggro; dawn calms them.
7. **Places you use** — Monolith hums; etch a wood pick into a faster break. Sunken ruin spawner emits hostiles. Sage sells the staff, the key, and a glowcap at the rates in §3. The vault stays shut until the key is consumed, then it is a chest. Glowworm cenote, fallen log, boulder, ossified ribs. Crystal grotto buds mine and place as light. Cave glowvines do too.
8. **Kit and kin** — Catch a glowbeetle; the lantern lights while held. Bees pollinate; breaking the hive stings and drops honeycomb. Tame and breed a horse; sprint drains stamina and sneak dismounts. Brew all three drinks and the heal. Fire the staff. Open pause → Advancements and see the list check off as you earn them. Cold-season tint and snowfall are on screen without changing the blocks. Biome fog differs between meadow and swamp. Wind moves the leaves. Skywhales pass, and goblins trail them. Crows call. A second process running `server` with the same seed shows the same mobs to a client.

## UI spec

### Job
A player in the world reads health, a boss, and a few short panels (etch, brew, trade, vault, advancements) without learning a new HUD. Success is the defining story with the same chrome they already use.

### Out
Not a title, settings, video, or pack menu. Not a map. Not a hunger or armor column. Not a skill-tree widget. Not a clock. Not a feywild screen. Those jobs do not exist here. Light, weather, and sway are the world canvas, not widgets.

### Topology
**Workspace** stays the play view. **Overlay** is the same interruption as today: one centered panel, then back to the world. Pause gains one more button that opens the advancement list and returns to pause. No new destination.

### Hierarchy
- Primary: the world, the mob in front of the crosshair, the boss bar when a boss is hurting you
- Secondary: hearts and hotbar
- Tertiary: F2 XYZ, pause seed, the advancement list, etch/brew/trade/vault slots

### Sibling
Closest shipped surface: play HUD and overlays (`crates/client/src/ui/hud.rs`, `crates/client/src/ui/container.rs`, `crates/client/src/ui/pause.rs`)
| Chrome / control / state | Keep / drop / adapt | Why |
| --- | --- | --- |
| World canvas + Crosshair | keep | Aim stays the interaction |
| HeartRow, Hotbar, DebugMeter | keep | Same HUD, same places |
| ContainerScreen + SlotGrid + Slot + Panel | adapt | Etch, brew, trade, and vault use this screen's slot language |
| PauseMenu (seed, Back to Game, Quit+confirm) | adapt | Add Advancements; seed and quit-confirm stay |
| Procedural atlas and skin fallback | keep | Missing art stays playable |
| Magenta missing texture | drop | Already banned; still banned |
| Boss bar | adapt | New composition on HudLayer, built like HeartRow from Panel and Text |
| Advancement list | adapt | New composition from the PauseMenu primitives |

### Kit
- Primitives: `Panel`, `Text`, `Button`, `Slot`, `Crosshair` (`crates/client/src/ui/prim.rs`)
- Compositions: `HudLayer`, `Hotbar`, `HeartRow`, `DebugMeter` (`hud.rs`); `ContainerScreen`, `SlotGrid` (`container.rs`); `PauseMenu` (`pause.rs`); `BossBar` (new composition in `hud.rs`, only `Panel` + `Text`); `AdvancementScreen` (new composition beside `PauseMenu`, only `Panel` + `Text` + `Button`)
- Recipes consulted (do not import): none
- Gaps (AskQuestion, do not invent): none

### Wire

Play:

```
┌ F2 DebugMeter (XYZ, off unless toggled)          ┐
│                                                   │
│     world (biomes, mobs, weather, sway, fog)     │
│                    + Crosshair                    │
│           BossBar (only during a boss)           │
│ HeartRow                         Hotbar          │
└───────────────────────────────────────────────────┘
```

Pause, then advancements:

```
┌──────── Game paused ─────────┐     ┌────── Advancements ──────┐
│ seed                          │     │ Wake in a meadow      ✓  │
│ [Back to Game]                │     │   Fell a tree            │
│ [Advancements]                │ --> │   Take a hide            │
│ [Quit] --> unsaved confirm    │     │ Ride                     │
└───────────────────────────────┘     │ [Back]                   │
                                      └──────────────────────────┘
```

Etch / brew / trade / vault: the centered `ContainerScreen` panel, hotbar along the bottom, one overlay at a time.

```
┌──────── panel ─────────┐
│  slots for this window │
│  [Etch] on etch only   │
│  Hotbar                │
└────────────────────────┘
```

### Regions

#### World canvas
- Job of this region: see and aim at the living world
- Contents: terrain, flora, mob skin-boxes, particles (pollen, fireflies, spores, ash, snow, rain, steam, leaves, aurora, shooting stars), fog tint, wind sway, blood-moon tint, season tint
- Job class: page
- Kit: world submit through the `engine-render` command API; fog, sway, and tints ride the existing uniform path the way brightness already does
- Not: a new scene graph; a post stack; a shadow-map redo
- Grouping: the world is the surface; HUD floats on it
- Named composition: none
- Inline: sun, brightness, biome fog, sway uniform, particle draws

#### Crosshair
- Job of this region: aim
- Contents: centered mark while pointer-locked; hidden in overlays
- Job class: badge
- Kit: `Crosshair`
- Not: a mouse cursor in play
- Grouping: alone at center
- Named composition: none
- Inline: the mark

#### BossBar
- Job of this region: show the treant or the Wyrm is still up, and how hurt it is
- Contents: the boss name and a single bar that shrinks as its health drops; absent for every other mob
- Job class: badge
- Kit: `BossBar` drawn by `HudLayer`, built from `Panel` and `Text`
- Not: a row of hearts; a corner portrait; a bar for goats
- Grouping: above HeartRow, only while that fight is active
- Named composition: `BossBar`
- Inline: the name and the fill

#### HeartRow
- Job of this region: show health
- Contents: 10 hearts, damage and heals visible immediately
- Job class: badge
- Kit: `HeartRow`
- Not: a text HP number; hunger
- Grouping: above the hotbar start
- Named composition: `HeartRow`
- Inline: heart pips

#### Hotbar
- Job of this region: choose what is in hand
- Contents: 9 slots, selected index, counts
- Job class: grid
- Kit: `Hotbar` wrapping `Slot`
- Not: a radial menu
- Grouping: bottom-center in play; the same row inside every container overlay
- Named composition: `Hotbar`
- Inline: selection highlight

#### DebugMeter
- Job of this region: read XYZ
- Contents: X/Y/Z when F2 is on; nothing else
- Job class: meta
- Kit: `DebugMeter`
- Not: a biome readout, an FPS counter, a clock
- Grouping: top-left
- Named composition: `DebugMeter`
- Inline: the three numbers

#### ContainerScreen
- Job of this region: move items; etch, brew, trade, or loot when that window is open
- Contents: etch is one tool slot plus an Etch `Button`; brew is three reagent slots and a result; trade is the sage's three offers; vault and ruin chests are the chest grid. Player storage and hotbar stay
- Job class: dialog
- Kit: `ContainerScreen` + `SlotGrid` + `Slot` + `Panel` + `Text`; Etch uses `Button`
- Not: a new inventory layout; egui; an anvil animation
- Grouping: the window's own slots above the player grid; hotbar last; Etch beside the tool slot, not in the hotbar
- Named composition: `ContainerScreen`
- Inline: the Etch label

#### PauseMenu
- Job of this region: stop, read the seed, open advancements, resume, or quit
- Contents: "Game paused", seed, Back to Game, Advancements, Quit then "World is not saved"
- Job class: dialog
- Kit: `PauseMenu` + `Panel` + `Text` + `Button`
- Not: settings; a title screen
- Grouping: seed with the title; Back, Advancements, and Quit stacked; confirm replaces that stack
- Named composition: `PauseMenu`
- Inline: the three actions

#### AdvancementScreen
- Job of this region: read what you have done
- Contents: the eight lines in §3, a mark on the ones the sim has granted, indented children, Back
- Job class: dialog
- Kit: `AdvancementScreen` from `Panel`, `Text`, `Button`
- Not: a node graph; a quest tracker on the HUD; checkboxes that are a new primitive
- Grouping: title, then the list, Back alone at the bottom
- Named composition: `AdvancementScreen`
- Inline: each line

### Layout
**Wide:** world fills the canvas; HUD pinned to the edges; one centered overlay at a time; boss bar centered above the hearts.
**Narrow:** still a desktop game; HUD scales with the existing `UiFrame` scale; panels stay centered.
**Touch:** not a touch product; click and the existing keys are the verbs; pointer lock in play, unlocked in overlays.

### States
| State | What the user sees | Sibling match |
| --- | --- | --- |
| Empty | Empty slots; advancement lines present and unchecked; no boss bar | keep — empty chrome is the empty |
| Loading | New chunks appear as you walk; no spinner | keep |
| Error | Missing art → procedural + stderr; missing sound → silence; vault without a key does not open | adapt — no error panel |
| Overflow | Stacks cap; leftovers stay item entities | keep |
| Permission | N/A for the local player; a vault without a key simply will not open | drop the dialog, keep the shut chest |
| Destructive | Quit still confirms the world is not saved; etching and drinking do not confirm | keep quit; no new confirms |

### Feedback
No toasts. Break and sting: particles and a positional call when a clip exists. Pickup, etch, brew, trade, and vault loot: the slot changes. Damage and heals: hearts. Boss: the bar. Catch, mount, root, and launch: the body and the world, no banner. Blood moon and seasons: the sky, continuous, no announcement.

### Anti-goals
egui. A map, clock, biome label, or FPS meter. A skill-tree canvas. Horse-armor slots. A portal screen. A brightness slider. Magenta placeholders. A boss bar for every mob. A second hotbar. Hover-only actions. A fluid-debug overlay.

## 9. Critical Failures

- A named part is missing or only a label, and it is not an intentional sibling-empty
- Built UI contradicts the UI spec (wrong topology, extra surfaces, missing regions, wrong kit export, omitted sibling `keep` chrome, a look that fights the spec)
- Existing solutions ignored for a parallel stack
- A harvested hard repo rule is violated
- A new primitive, recipe, crate, dep, thread, backend, or data model shipped without the required stop-and-ask
- Built solo, or several named features folded into one subagent. One feature, one subagent, same working tree. A per-feature branch or worktree is a failed run
- Declared done without a live client pass and without the user clicking the §8 views
- Concealment (mocks, placeholders, fake evidence files, mobs that do not tick on the server)
- A second world, portal, or feywild
- A second character controller, or a second voxel store for fluids or biomes
- An ECS, AI crate, or flowing-fluid simulator
- Mojang/Microsoft assets fetched or committed
- Tick rate or day length changed
- Horse armor, hunger, or a title menu sneaking in
- Magenta art, or a crash on a missing clip or sprite
- Spawn that is not the sunpetal meadow, or seed `1234` that hides a named view past a few minutes of walking, one cave, or the beanstalk
- Farm mobs that stay hostile after dawn when it is not a blood moon
- A griffin that hovers, or a geyser that skips fall damage
- Etch, brew, trade, tame, vault, or the Wyrm resolved on the client alone

## 10. Deliverables

The product in this repo. Nothing else. No mission README, evidence pack, or recovery markdown.
