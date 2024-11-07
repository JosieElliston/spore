// i really don't need a good rng
// i even don't need it to be very uniform
// i just want it to be fast

// the java.Random rng
// https://docs.oracle.com/javase/6/docs/api/java/util/Random.html
// generally try to take higher bits of the seed
pub struct Rng {
    seed: u64,
}

impl Rng {
    pub fn seeded() -> Self {
        Self {
            seed: std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        }
    }

    pub fn next(&mut self, bits: u8) -> u32 {
        debug_assert!((1..=32).contains(&bits));
        self.seed = (self.seed.wrapping_mul(0x5DEECE66D).wrapping_add(0xB)) & ((1 << 48) - 1);
        (self.seed >> (48 - bits)) as u32
    }

    // pub fn next_u32(&mut self) -> u32 {
    //     self.next(32)
    // }

    pub fn next_u32_n(&mut self, max: u32) -> u32 {
        if is_pow_of_two_or_zero(max) {
            return (((max as u64) * (self.next(31) as u64)) >> 31) as u32;
        }
        loop {
            let bits = self.next(31);
            let val = bits % max;
            if (bits as i32).wrapping_sub(val as i32).wrapping_add(max as i32) > 0 {
                return val;
            }
        }
    }

    // pub fn next_u32_n_fast(&mut self, max: u32) -> u32 {
    //     if is_pow_of_two_or_zero(max) {
    //         (((max as u64) * (self.next(31) as u64)) >> 31) as u32
    //     } else {
    //         self.next(31) % max
    //     }
    // }
}

const fn is_pow_of_two_or_zero(n: u32) -> bool {
    n & (n - 1) == 0
}
