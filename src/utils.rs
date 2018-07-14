extern crate fixedbitset;

use fixedbitset::FixedBitSet;

pub fn get_next_set(bitset: &FixedBitSet, index: usize) -> u64 {
    for i in index..bitset.len() {
        if bitset.contains(i) {
            return i as u64;
        }
    }
    0
}

pub fn set_bits(b: &mut FixedBitSet, offset: u64, bits: u64, length: u64) {
    for i in 0..length {
        let val = bits & (1 << (length - i - 1));
        b.set((offset + i + 1) as usize, val > 0);
    }
}

pub fn round(a: f64) -> u64 {
    if a < 0_f64 {
        (a - 0.5) as u64
    } else {
        (a + 0.5) as u64
    }
}

pub fn msb(x: u64) -> u64 {
    (round((x as f64).log2())) as u64
}
