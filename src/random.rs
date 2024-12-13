use tinyrand::{Rand, StdRand};

pub struct RNG {
    rand: StdRand,
}

impl RNG {
    pub fn new() -> Self {
        Self {
            rand: StdRand::default(),
        }
    }

    pub fn next_u8(&mut self) -> u8 {
        let r_u16 = self.rand.next_u16();
        r_u16 as u8
    }
}
