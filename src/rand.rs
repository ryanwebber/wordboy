use crate::mmio::REG_VCOUNT;

pub struct PRNG(u16);

impl PRNG {
    const A: u16 = 48271; // Multiplier (commonly used in LCGs)
    const C: u16 = 0; // Increment (can be 0 for minimal LCG)
    const M: u16 = u16::MAX; // Modulus for u16 (65535)

    pub fn new(seed: u16) -> Self {
        PRNG(seed)
    }

    pub fn seeded() -> Self {
        let seed = REG_VCOUNT.read();
        PRNG(seed)
    }

    // Generate the next pseudo-random number
    pub fn next(&mut self) -> u16 {
        // Calculate the next number in the sequence
        self.0 = (Self::A.wrapping_mul(self.0).wrapping_add(Self::C)) % Self::M;
        self.0
    }
}
