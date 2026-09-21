//! Biome plants and trunks. Terrain shape stays in worldgen.

use world::Chunk;

mod meadow;
mod oak;
mod birch;
mod pine;
mod redwood;
mod dune;
mod mesa;
mod savanna;
mod jungle;
mod swamp;
mod mushroom;
mod coral;
mod ash;
mod frost;
mod glow;
mod moor;

pub fn grow_all(chunk: &mut Chunk, seed: i64) {
    meadow::grow(chunk, seed);
    oak::grow(chunk, seed);
    birch::grow(chunk, seed);
    pine::grow(chunk, seed);
    redwood::grow(chunk, seed);
    dune::grow(chunk, seed);
    mesa::grow(chunk, seed);
    savanna::grow(chunk, seed);
    jungle::grow(chunk, seed);
    swamp::grow(chunk, seed);
    mushroom::grow(chunk, seed);
    coral::grow(chunk, seed);
    ash::grow(chunk, seed);
    frost::grow(chunk, seed);
    glow::grow(chunk, seed);
    moor::grow(chunk, seed);
}
