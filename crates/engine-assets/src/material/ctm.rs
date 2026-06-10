/// Cardinal neighbor bitmask for connected-texture lookup keys (packed at asset time).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NeighborMask {
    bits: u8,
}

impl NeighborMask {
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    pub fn bits(self) -> u8 {
        self.bits
    }

    pub fn from_cardinals(north: bool, east: bool, south: bool, west: bool) -> Self {
        let mut bits = 0u8;
        if north {
            bits |= 1;
        }
        if east {
            bits |= 2;
        }
        if south {
            bits |= 4;
        }
        if west {
            bits |= 8;
        }
        Self { bits }
    }
}
