# Block material engine

**Status:** Living document for Minecraft-parity materials (pre-import).

Authority: [`design-doc.md`](./design-doc.md) for render boundaries; this doc defines material data flow and milestone gates.

## ResolvedFace contract

At pack time, each `(block_id, CubeFace)` maps to a `ResolvedFace`:

- `atlas_rect` — UV rectangle in the block atlas
- `draw_category` — `Opaque`, `Cutout`, or `Transparent` (reserved)
- `uv2` — optional second atlas rect for overlays (M3+)
- `tint` — `None`, `BiomeGrass`, or `BiomeFoliage` (M6+)
- `anim` — optional animation strip metadata (M7+)

Runtime resolution uses **table lookup only**: `(id, state, face, NeighborMask?) → ResolvedFace`. No open-ended context objects.

## Layer boundaries

| Crate | Owns | Must NOT |
|-------|------|----------|
| `engine-assets` | `BlockDefinition`, pack → `ResolvedBlockMaterials` | Read SVO, biome, neighbors |
| `engine-world` | `VoxelCell`, `BlockState`, `BiomeMap` | Render types |
| `client` extract | `mesh_chunk` → `RenderWorld` | Game logic |
| `engine-render` | GPU upload and draw passes | Registry, SVO, biome at draw time |

## RenderWorld evolution

| Milestone | Fields |
|-----------|--------|
| M1 | `camera`, `meshes` (opaque only) |
| M2 | `opaque_meshes`, `cutout_meshes` |
| M3+ | `MeshVertex.uv2`, overlay flag |
| M6 | per-vertex `tint_index` |
| M7 | `animation_tick` uniform |

## Atlas budget

- Tile size: 16×16
- Default grid: 16×16 (256×256 atlas)
- Packing fails loudly when tile slots are exhausted (dev builds panic via `expect` on pack errors)
- Animation strips consume multiple contiguous horizontal tiles

## Network (M4+)

`BlockDelta` includes optional `state: u8` (default 0). Server remains authoritative.

## CTM dirty policy (M5+)

When a block with connected textures changes, invalidate chunk meshes in a **2-block halo** around the edit (not only 3×3×3 chunks).

## CPU mesher

Chunk meshing runs on the main thread during Extract. Material resolution API must remain callable from extract workers when GPU compute meshing arrives—resolution is separate from draw.

## Milestones

| ID | Gate |
|----|------|
| M0 | This document + implementation-plan Phase 18 |
| M1 | Static `ResolvedFace` tables; visual parity grass/dirt/stone |
| M2 | Opaque + cutout draw buckets and pipeline passes |
| M3 | Runtime grass side overlay (`uv2`) |
| M4 | `VoxelCell` + state variant tables |
| M5 | CTM neighbor masks |
| M6 | Biome colormap tint at mesh time |
| M7 | Texture animation from mcmeta |

## Texture pack importer (Phase 19)

Curated import from a Minecraft resource pack (zip or extracted folder) into engine assets:

```bash
cargo run -p engine-assets --bin import-texture-pack -- \
  --pack source-packs/whimscape-26.1-r2/whimscape-26.1-r2.zip \
  --assets /path/to/assets
```

Archived upstream packs: [`source-packs/`](../source-packs/README.md).

Manifest: [`assets/import/manifest.toml`](../assets/import/manifest.toml) maps engine block names → MC model ids (e.g. `grass_block_alt_1`). The importer:

1. Resolves `cube_all` / grass-block / leaves models to per-face 16×16 textures
2. Composes `cube_v1` `albedo.png` (64×32 cross-net) per block
3. Writes runtime `overlay.png` for grass sides when the model declares an overlay layer
4. Copies `colormap/grass.png` and `colormap/foliage.png`
5. Patches existing `assets/blocks/{name}.toml` (`draw`, `tint`, `[[overlays]]`)

Block ids and registry entries must exist before import. Bulk auto-registration is out of scope.
