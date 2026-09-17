use std::fmt;

/// Generational entity ID. Index is dense; generation invalidates freed slots.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityId {
    pub index: u32,
    pub gen: u32,
}

impl EntityId {
    pub const fn new(index: u32, gen: u32) -> Self {
        Self { index, gen }
    }

    pub const fn is_null(self) -> bool {
        self.gen == 0
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.index, self.gen)
    }
}
