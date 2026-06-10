use crate::block::BlockPos;

/// Column biome ids for tinting (M6). Stub varies by column until real biome gen lands.
#[derive(Debug, Default, Clone)]
pub struct BiomeMap;

impl BiomeMap {
    pub fn biome_at(&self, position: BlockPos) -> u8 {
        let x = position.0.x.wrapping_abs();
        let y = position.0.y.wrapping_abs();
        let index = (x.wrapping_mul(374761) ^ y.wrapping_mul(668265)) & 0xFF;
        (index as u8).max(1)
    }
}
