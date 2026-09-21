//! One file per mob. The sim calls `decide`.

use world::EntityKind;

use crate::mind::{Sight, Will};

pub mod aurochs;
pub mod boar;
pub mod goat;
pub mod rabbit;
pub mod deer;
pub mod fox;
pub mod songbird;
pub mod crow;
pub mod fish;
pub mod frog;
pub mod glowbeetle;
pub mod bee;
pub mod horse;
pub mod griffin;
pub mod goblin;
pub mod husk;
pub mod wraith;
pub mod wight;
pub mod imp;
pub mod hopper;
pub mod mimic;
pub mod sentinel;
pub mod treant;
pub mod skywhale;
pub mod wyrm;
pub mod sage;
pub mod parrot;

pub fn decide(kind: EntityKind, s: &Sight) -> Will {
    match kind {
        EntityKind::Aurochs => aurochs::think(s),
        EntityKind::Boar => boar::think(s),
        EntityKind::Goat => goat::think(s),
        EntityKind::Rabbit => rabbit::think(s),
        EntityKind::Deer => deer::think(s),
        EntityKind::Fox => fox::think(s),
        EntityKind::Songbird => songbird::think(s),
        EntityKind::Crow => crow::think(s),
        EntityKind::Fish => fish::think(s),
        EntityKind::Frog => frog::think(s),
        EntityKind::Glowbeetle => glowbeetle::think(s),
        EntityKind::Bee => bee::think(s),
        EntityKind::Horse => horse::think(s),
        EntityKind::Griffin => griffin::think(s),
        EntityKind::Goblin => goblin::think(s),
        EntityKind::Husk => husk::think(s),
        EntityKind::Wraith => wraith::think(s),
        EntityKind::Wight => wight::think(s),
        EntityKind::Imp => imp::think(s),
        EntityKind::Hopper => hopper::think(s),
        EntityKind::Mimic => mimic::think(s),
        EntityKind::Sentinel => sentinel::think(s),
        EntityKind::Treant => treant::think(s),
        EntityKind::Skywhale => skywhale::think(s),
        EntityKind::Wyrm => wyrm::think(s),
        EntityKind::Sage => sage::think(s),
        EntityKind::Parrot => parrot::think(s),
        EntityKind::Player | EntityKind::Item | EntityKind::Projectile => Will::hold(s),
    }
}
