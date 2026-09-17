const AIR: u16 = 0;
const SIZE: usize = 16 * 16 * 16;

/// Paletted 16³ section. Single-value (air) until written.
#[derive(Clone, Debug)]
pub struct Section {
    palette: Vec<u16>,
    /// Index into palette per voxel. Empty vec means all `palette[0]`.
    index: Vec<u16>,
}

impl Section {
    pub fn air() -> Self {
        Self {
            palette: vec![AIR],
            index: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.palette.len() == 1 && self.palette[0] == AIR && self.index.is_empty()
    }

    pub fn get(&self, i: usize) -> u16 {
        if self.index.is_empty() {
            return self.palette[0];
        }
        let pal = self.index[i] as usize;
        self.palette[pal]
    }

    pub fn set(&mut self, i: usize, id: u16) {
        if self.index.is_empty() {
            if id == self.palette[0] {
                return;
            }
            self.index = vec![0; SIZE];
        }
        let pal = self.ensure_pal(id);
        self.index[i] = pal;
    }

    fn ensure_pal(&mut self, id: u16) -> u16 {
        if let Some(i) = self.palette.iter().position(|p| *p == id) {
            return i as u16;
        }
        let i = self.palette.len() as u16;
        self.palette.push(id);
        i
    }

    pub fn palette(&self) -> &[u16] {
        &self.palette
    }

    pub fn indices(&self) -> &[u16] {
        &self.index
    }

    pub fn from_parts(palette: Vec<u16>, index: Vec<u16>) -> Self {
        let palette = if palette.is_empty() { vec![AIR] } else { palette };
        Self { palette, index }
    }

    pub fn fill(&self) -> [u16; SIZE] {
        let mut out = [AIR; SIZE];
        for i in 0..SIZE {
            out[i] = self.get(i);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get() {
        let mut s = Section::air();
        assert_eq!(s.get(0), 0);
        s.set(3, 7);
        assert_eq!(s.get(3), 7);
        assert_eq!(s.get(0), 0);
    }
}
