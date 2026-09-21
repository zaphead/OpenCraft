//! Seeded set-pieces. Each file stamps one.

use world::Chunk;

mod fairy;
mod monolith;
mod sunken;
mod temple;
mod tower;
mod camp;
mod beanstalk;
mod arch;
mod tavolo;
mod canyon;
mod crater;
mod spring;
mod geyser;
mod thorn;
mod litter;
mod cenote;
mod rainbow;
mod grotto;
mod graves;
mod wyrmpit;

pub fn stamp_all(chunk: &mut Chunk, seed: i64) {
    fairy::stamp(chunk, seed);
    monolith::stamp(chunk, seed);
    sunken::stamp(chunk, seed);
    temple::stamp(chunk, seed);
    tower::stamp(chunk, seed);
    camp::stamp(chunk, seed);
    beanstalk::stamp(chunk, seed);
    arch::stamp(chunk, seed);
    tavolo::stamp(chunk, seed);
    canyon::stamp(chunk, seed);
    crater::stamp(chunk, seed);
    spring::stamp(chunk, seed);
    geyser::stamp(chunk, seed);
    thorn::stamp(chunk, seed);
    litter::stamp(chunk, seed);
    cenote::stamp(chunk, seed);
    rainbow::stamp(chunk, seed);
    grotto::stamp(chunk, seed);
    graves::stamp(chunk, seed);
    wyrmpit::stamp(chunk, seed);
}
