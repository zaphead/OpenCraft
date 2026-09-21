//! AABB, voxel DDA, stepped collision, Java-like body clock.

mod aabb;
mod body;
mod collide;
mod ray;

pub use aabb::Aabb;
pub use body::{
    take_landed_fall, tick_body, Body, Fluid, MoveInput, Ride, PLAYER_EYE, PLAYER_HEIGHT,
    PLAYER_SNEAK_EYE, PLAYER_SNEAK_HEIGHT, PLAYER_WIDTH,
};
pub use collide::{move_colliding, VoxelSolid};
pub use ray::{hit_voxel, RayHit};
