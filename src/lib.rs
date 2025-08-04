#![cfg_attr(not(test), no_std)]
const SQ5_BIT_NOISE1: u32 = 0xd2a80a3f; // 11010010101010000000101000111111
const SQ5_BIT_NOISE2: u32 = 0xa884f197; // 10101000100001001111000110010111
const SQ5_BIT_NOISE3: u32 = 0x6C736F4B; // 01101100011100110110111101001011
const SQ5_BIT_NOISE4: u32 = 0xB79F3ABB; // 10110111100111110011101010111011
const SQ5_BIT_NOISE5: u32 = 0x1b56c4f5; // 00011011010101101100010011110101

const PRIME1: i32 = 198491317; // Large prime number with non-boring bits
const PRIME2: i32 = 6542989; // Large prime number with distinct and non-boring bits
const PRIME3: i32 = 357239; // Large prime number with distinct and non-boring bits

#[inline]
pub fn squirrel_noise5(index: u32, seed: u32) -> u32 {
    let mut mangled_bits = index;

    mangled_bits = mangled_bits.wrapping_mul(SQ5_BIT_NOISE1);
    mangled_bits = mangled_bits.wrapping_add(seed);
    mangled_bits ^= mangled_bits >> 9;
    mangled_bits = mangled_bits.wrapping_add(SQ5_BIT_NOISE2);
    mangled_bits ^= mangled_bits >> 11;
    mangled_bits = mangled_bits.wrapping_mul(SQ5_BIT_NOISE3);
    mangled_bits ^= mangled_bits >> 13;
    mangled_bits = mangled_bits.wrapping_add(SQ5_BIT_NOISE4);
    mangled_bits ^= mangled_bits >> 15;
    mangled_bits = mangled_bits.wrapping_mul(SQ5_BIT_NOISE5);
    mangled_bits ^= mangled_bits >> 17;

    mangled_bits
}

#[inline]
fn linearize_2d(x: i32, y: i32) -> i32 {
    x.wrapping_add(y.wrapping_mul(PRIME1))
}
#[inline]
fn linearize_3d(x: i32, y: i32, z: i32) -> i32 {
    x.wrapping_add(y.wrapping_mul(PRIME1))
        .wrapping_add(z.wrapping_mul(PRIME2))
}
#[inline]
fn linearize_4d(x: i32, y: i32, z: i32, w: i32) -> i32 {
    x.wrapping_add(y.wrapping_mul(PRIME1))
        .wrapping_add(z.wrapping_mul(PRIME2))
        .wrapping_add(w.wrapping_mul(PRIME3))
}
//-------------------------------
//      u32
//-------------------------------
#[inline]
pub fn u32_1d(index: i32, seed: i32) -> u32 {
    squirrel_noise5(index as u32, seed as u32)
}
#[inline]
pub fn u32_2d(x: i32, y: i32, seed: i32) -> u32 {
    squirrel_noise5(linearize_2d(x, y) as u32, seed as u32)
}
#[inline]
pub fn u32_3d(x: i32, y: i32, z: i32, seed: i32) -> u32 {
    squirrel_noise5(linearize_3d(x, y, z) as u32, seed as u32)
}
#[inline]
pub fn u32_4d(x: i32, y: i32, z: i32, w: i32, seed: i32) -> u32 {
    squirrel_noise5(linearize_4d(x, y, z, w) as u32, seed as u32)
}

//-------------------------------
//      u32_range
//-------------------------------
#[inline]
pub fn u32_range_1d(min: u32, max: u32, index: i32, seed: i32) -> u32 {
    min + ((max - min) as f32 * f32_zero_to_one_1d(index, seed)) as u32
}
#[inline]
pub fn u32_range_2d(min: u32, max: u32, x: i32, y: i32, seed: i32) -> u32 {
    min + ((max - min) as f32 * f32_zero_to_one_2d(x, y, seed)) as u32
}
#[inline]
pub fn u32_range_3d(min: u32, max: u32, x: i32, y: i32, z: i32, seed: i32) -> u32 {
    min + ((max - min) as f32 * f32_zero_to_one_3d(x, y, z, seed)) as u32
}
#[inline]
pub fn u32_range_4d(min: u32, max: u32, x: i32, y: i32, z: i32, w: i32, seed: i32) -> u32 {
    min + ((max - min) as f32 * f32_zero_to_one_4d(x, y, z, w, seed)) as u32
}

//-------------------------------
//      u32_cap
//-------------------------------
#[inline]
pub fn u32_cap_1d(max: u32, index: i32, seed: i32) -> u32 {
    (max as f32 * f32_zero_to_one_1d(index, seed)) as u32
}
#[inline]
pub fn u32_cap_2d(max: u32, x: i32, y: i32, seed: i32) -> u32 {
    (max as f32 * f32_zero_to_one_2d(x, y, seed)) as u32
}
#[inline]
pub fn u32_cap_3d(max: u32, x: i32, y: i32, z: i32, seed: i32) -> u32 {
    (max as f32 * f32_zero_to_one_3d(x, y, z, seed)) as u32
}
#[inline]
pub fn u32_cap_4d(max: u32, x: i32, y: i32, z: i32, w: i32, seed: i32) -> u32 {
    (max as f32 * f32_zero_to_one_4d(x, y, z, w, seed)) as u32
}

//-------------------------------
//      u64
//-------------------------------
#[inline]
pub fn u64_1d(index: i32, seed: i32) -> u64 {
    let fst = u32_1d(index, seed);
    let snd = u32_1d(index, fst as i32);
    ((fst as u64) << 32) | snd as u64
}
#[inline]
pub fn u64_2d(x: i32, y: i32, seed: i32) -> u64 {
    u64_1d(linearize_2d(x, y), seed)
}
#[inline]
pub fn u64_3d(x: i32, y: i32, z: i32, seed: i32) -> u64 {
    u64_1d(linearize_3d(x, y, z), seed)
}
#[inline]
pub fn u64_4d(x: i32, y: i32, z: i32, w: i32, seed: i32) -> u64 {
    u64_1d(linearize_4d(x, y, z, w), seed)
}

//-------------------------------
//      i32
//-------------------------------
#[inline]
pub fn i32_1d(index: i32, seed: i32) -> i32 {
    u32_1d(index, seed) as i32
}
#[inline]
pub fn i32_2d(x: i32, y: i32, seed: i32) -> i32 {
    u32_2d(x, y, seed) as i32
}
#[inline]
pub fn i32_3d(x: i32, y: i32, z: i32, seed: i32) -> i32 {
    u32_3d(x, y, z, seed) as i32
}
#[inline]
pub fn i32_4d(x: i32, y: i32, z: i32, w: i32, seed: i32) -> i32 {
    u32_4d(x, y, z, w, seed) as i32
}

//-------------------------------
//      i32_range
//-------------------------------
#[inline]
pub fn i32_range_1d(min: i32, max: i32, index: i32, seed: i32) -> i32 {
    min + ((max - min) as f32 * f32_zero_to_one_1d(index, seed)) as i32
}
#[inline]
pub fn i32_range_2d(min: i32, max: i32, x: i32, y: i32, seed: i32) -> i32 {
    min + ((max - min) as f32 * f32_zero_to_one_2d(x, y, seed)) as i32
}
#[inline]
pub fn i32_range_3d(min: i32, max: i32, x: i32, y: i32, z: i32, seed: i32) -> i32 {
    min + ((max - min) as f32 * f32_zero_to_one_3d(x, y, z, seed)) as i32
}
#[inline]
pub fn i32_range_4d(min: i32, max: i32, x: i32, y: i32, z: i32, w: i32, seed: i32) -> i32 {
    min + ((max - min) as f32 * f32_zero_to_one_4d(x, y, z, w, seed)) as i32
}

//-------------------------------
//      i32_cap
//-------------------------------
#[inline]
pub fn i32_cap_1d(max: i32, index: i32, seed: i32) -> i32 {
    (max as f32 * f32_zero_to_one_1d(index, seed)) as i32
}
#[inline]
pub fn i32_cap_2d(max: i32, x: i32, y: i32, seed: i32) -> i32 {
    (max as f32 * f32_zero_to_one_2d(x, y, seed)) as i32
}
#[inline]
pub fn i32_cap_3d(max: i32, x: i32, y: i32, z: i32, seed: i32) -> i32 {
    (max as f32 * f32_zero_to_one_3d(x, y, z, seed)) as i32
}
#[inline]
pub fn i32_cap_4d(max: i32, x: i32, y: i32, z: i32, w: i32, seed: i32) -> i32 {
    (max as f32 * f32_zero_to_one_4d(x, y, z, w, seed)) as i32
}

//-------------------------------
//      i64
//-------------------------------
#[inline]
pub fn i64_1d(index: i32, seed: i32) -> i64 {
    u64_1d(index, seed) as i64
}
#[inline]
pub fn i64_2d(x: i32, y: i32, seed: i32) -> i64 {
    u64_2d(x, y, seed) as i64
}
#[inline]
pub fn i64_3d(x: i32, y: i32, z: i32, seed: i32) -> i64 {
    u64_3d(x, y, z, seed) as i64
}
#[inline]
pub fn i64_4d(x: i32, y: i32, z: i32, w: i32, seed: i32) -> i64 {
    u64_4d(x, y, z, w, seed) as i64
}

//-------------------------------
//      f32_range
//-------------------------------
#[inline]
pub fn f32_range_1d(min: f32, max: f32, index: i32, seed: i32) -> f32 {
    min + (max - min) * f32_zero_to_one_1d(index, seed)
}
#[inline]
pub fn f32_range_2d(min: f32, max: f32, x: i32, y: i32, seed: i32) -> f32 {
    min + (max - min) * f32_zero_to_one_2d(x, y, seed)
}
#[inline]
pub fn f32_range_3d(min: f32, max: f32, x: i32, y: i32, z: i32, seed: i32) -> f32 {
    min + (max - min) * f32_zero_to_one_3d(x, y, z, seed)
}
#[inline]
pub fn f32_range_4d(min: f32, max: f32, x: i32, y: i32, z: i32, w: i32, seed: i32) -> f32 {
    min + (max - min) * f32_zero_to_one_4d(x, y, z, w, seed)
}

//-------------------------------
//      f32_01
//-------------------------------
#[inline]
pub fn f32_zero_to_one_1d(index: i32, seed: i32) -> f32 {
    (u32_1d(index, seed) as f64 / u32::MAX as f64) as f32
}
#[inline]
pub fn f32_zero_to_one_2d(x: i32, y: i32, seed: i32) -> f32 {
    (u32_2d(x, y, seed) as f64 / u32::MAX as f64) as f32
}
#[inline]
pub fn f32_zero_to_one_3d(x: i32, y: i32, z: i32, seed: i32) -> f32 {
    (u32_3d(x, y, z, seed) as f64 / u32::MAX as f64) as f32
}
#[inline]
pub fn f32_zero_to_one_4d(x: i32, y: i32, z: i32, w: i32, seed: i32) -> f32 {
    (u32_4d(x, y, z, w, seed) as f64 / u32::MAX as f64) as f32
}

//-------------------------------
//      f32_-11
//-------------------------------
#[inline]
pub fn f32_neg_one_to_one_1d(index: i32, seed: i32) -> f32 {
    (i32_1d(index, seed) as f64 / i32::MAX as f64) as f32
}
#[inline]
pub fn f32_neg_one_to_one_2d(x: i32, y: i32, seed: i32) -> f32 {
    (i32_2d(x, y, seed) as f64 / i32::MAX as f64) as f32
}
#[inline]
pub fn f32_neg_one_to_one_3d(x: i32, y: i32, z: i32, seed: i32) -> f32 {
    (i32_3d(x, y, z, seed) as f64 / i32::MAX as f64) as f32
}
#[inline]
pub fn f32_neg_one_to_one_4d(x: i32, y: i32, z: i32, w: i32, seed: i32) -> f32 {
    (i32_4d(x, y, z, w, seed) as f64 / i32::MAX as f64) as f32
}

use core::cell::Cell;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct SquirrelRng {
    pub seed: Cell<i32>,
    pub index: Cell<i32>,
}

impl Default for SquirrelRng {
    fn default() -> Self {
        SquirrelRng::from_pos_and_seed(0, 0)
    }
}

#[cfg(feature = "rand_squirrel")]
pub use rand::{Rng, RngCore, SeedableRng};

#[cfg(feature = "rand_squirrel")]
impl SquirrelRng {
    pub fn new() -> Self {
        SquirrelRng {
            seed: Cell::from(rand::rng().random()),
            index: Cell::default(),
        }
    }

    pub fn with_seed(seed: i32) -> Self {
        Self {
            index: Cell::from(0),
            seed: Cell::from(seed),
        }
    }

    pub fn with_position(self, index: i32) -> Self {
        Self {
            index: Cell::from(index),
            ..self
        }
    }
}

#[cfg(feature = "rand_squirrel")]
impl RngCore for SquirrelRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.u32()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.u64()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_via_next(self, dest);
    }
}

#[cfg(feature = "rand_squirrel")]
fn fill_bytes_via_next<R: RngCore + ?Sized>(rng: &mut R, dest: &mut [u8]) {
    let mut left = dest;
    while left.len() >= 8 {
        let (l, r) = { left }.split_at_mut(8);
        left = r;
        let chunk: [u8; 8] = rng.next_u64().to_le_bytes();
        l.copy_from_slice(&chunk);
    }
    let n = left.len();
    if n > 4 {
        let chunk: [u8; 8] = rng.next_u64().to_le_bytes();
        left.copy_from_slice(&chunk[..n]);
    } else if n > 0 {
        let chunk: [u8; 4] = rng.next_u32().to_le_bytes();
        left.copy_from_slice(&chunk[..n]);
    }
}

#[cfg(feature = "rand_squirrel")]
impl SeedableRng for SquirrelRng {
    type Seed = [u8; 4];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::with_seed(i32::from_le_bytes(seed))
    }
}

impl SquirrelRng {
    #[inline]
    pub fn from_pos_and_seed(seed: i32, index: i32) -> Self {
        SquirrelRng {
            seed: Cell::from(seed),
            index: Cell::from(index),
        }
    }
    #[inline]
    pub fn u32(&self) -> u32 {
        let res = u32_1d(self.index.get(), self.seed.get());
        self.index.update(|x| x.wrapping_add(1));
        res
    }
    #[inline]
    pub fn u32_range(&self, min: u32, max: u32) -> u32 {
        let res = u32_range_1d(min, max, self.index.get(), self.seed.get());
        self.index.update(|x| x.wrapping_add(1));
        res
    }
    #[inline]
    pub fn u32_cap(&self, max: u32) -> u32 {
        let res = u32_cap_1d(max, self.index.get(), self.seed.get());
        self.index.update(|x| x.wrapping_add(1));
        res
    }
    #[inline]
    pub fn u64(&self) -> u64 {
        let res = u64_1d(self.index.get(), self.seed.get());
        self.index.update(|x| x.wrapping_add(1));
        res
    }

    #[inline]
    pub fn i32(&self) -> i32 {
        let res = i32_1d(self.index.get(), self.seed.get());
        self.index.update(|x| x.wrapping_add(1));
        res
    }
    #[inline]
    pub fn i32_range(&self, min: i32, max: i32) -> i32 {
        let res = i32_range_1d(min, max, self.index.get(), self.seed.get());
        self.index.update(|x| x.wrapping_add(1));
        res
    }
    #[inline]
    pub fn i32_cap(&self, max: i32) -> i32 {
        let res = i32_cap_1d(max, self.index.get(), self.seed.get());
        self.index.update(|x| x.wrapping_add(1));
        res
    }
    #[inline]
    pub fn i64(&self) -> i64 {
        let res = i64_1d(self.index.get(), self.seed.get());
        self.index.update(|x| x.wrapping_add(1));
        res
    }

    #[inline]
    pub fn usize(&self, max: usize) -> usize {
        (self.f32_zero_to_one() * max as f32) as usize
    }
    #[inline]
    pub fn usize_range(&self, min: usize, max: usize) -> usize {
        min + self.usize(max - min)
    }

    #[inline]
    pub fn u8(&self) -> u8 {
        (self.f32_zero_to_one() * 256_f32) as u8
    }
    #[inline]
    pub fn u8_cap(&self, max: u8) -> u8 {
        (self.f32_zero_to_one() * max as f32) as u8
    }
    #[inline]
    pub fn u8_range(&self, min: u8, max: u8) -> u8 {
        min + self.u8_cap(max - min)
    }

    #[inline]
    pub fn bool(&self) -> bool {
        self.i32() < 0
    }

    #[inline]
    pub fn f32_range(&self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.f32_zero_to_one()
    }
    #[inline]
    pub fn f32_zero_to_one(&self) -> f32 {
        let res = f32_zero_to_one_1d(self.index.get(), self.seed.get());
        self.index.update(|x| x.wrapping_add(1));
        res
    }
    #[inline]
    pub fn f32_neg_one_to_one(&self) -> f32 {
        let res = f32_neg_one_to_one_1d(self.index.get(), self.seed.get());
        self.index.update(|x| x.wrapping_add(1));
        res
    }

    #[inline]
    pub fn shuffle<T>(&self, slice: &mut [T]) {
        for n in 1..slice.len() {
            slice.swap(n, self.usize(n + 1));
        }
    }

    #[inline]
    pub fn jump(&self, amount: i32) {
        self.index.update(|x| x.wrapping_add(amount));
    }
    #[inline]
    pub fn goto(&self, index: i32) {
        self.index.set(index);
    }
    #[inline]
    pub fn seed(&self, seed: i32) {
        self.seed.set(seed);
    }
}

#[cfg(test)]
mod tests {
    use crate::u32_1d;

    const TEST_CASES: &[(i32, i32, u32)] = &[
        (0, 0, 0x16791E00),
        (1, 0, 0xC895CB1D),
        (-1, 0, 0xFAF16D54),
        (123, 456, 0x0771723F),
        (-123, 456, 0x09B50E33),
        (2147483647, -1, 0x1697A56A),
        (-2147483648, 0, 0x679CCD13),
        (42, 1337, 0x968DE4C9),
        (9999, 9999, 0x173A5069),
        (314159, 271828, 0xEFA3B8DC),
    ];
    #[test]
    fn known_values() {
        for &(index, seed, expected) in TEST_CASES {
            let result = u32_1d(index, seed);
            assert_eq!(
                result, expected,
                "Failed for index: {index}, seed: {seed}. Expected: {expected:#010X}, Got: {result:#010X}",
            );
        }
    }
}
