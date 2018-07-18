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
        if offset + i + 1 >= b.len() as u64 {
            let new_length = offset + i + 1 + 1;
            b.grow(new_length as usize);
        }
        b.set((offset + i + 1) as usize, val > 0);
    }
}

pub fn msb(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        63u64 - n.leading_zeros() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msb() {
        assert_eq!(0, msb(0));
        assert_eq!(0, msb(1));
        assert_eq!(1, msb(3));
        assert_eq!(4, msb(30));
        assert_eq!(6, msb(91));
        assert_eq!(63, msb(<u64>::max_value()));
    }
}

// "It is non trivial whether this code is actually correct for all u64 :
// https://github.com/tomarrell/rust-elias-fano/blob/master/src/utils.rs#L33
// u64::heading_zeros is a better implementation of msb. Again it compiles to one instruction, and it does not require to study f64 and its log implementation to know whether it is correct or not." - fulmicoton
