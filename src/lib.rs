extern crate fixedbitset;

mod utils;

use utils::*;

use fixedbitset::FixedBitSet;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EliasFano {
    universe: u64,
    n: u64,
    lower_bits: u64,
    higher_bits_length: u64,
    mask: u64,
    lower_bits_offset: u64,
    bv_len: u64,
    b: FixedBitSet,
    cur_value: u64,
    position: u64,
    high_bits_pos: u64,
}

#[derive(Debug)]
pub struct OutOfBoundsError;

impl Error for OutOfBoundsError {
    fn description(&self) -> &str {
        "Index out of bounds"
    }
}

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index out of range attempted to be accessed")
    }
}

impl EliasFano {
    pub fn new(universe: u64, n: u64) -> EliasFano {
        let lower_bits = if universe > n { msb(universe / n) } else { 0 };
        let higher_bits_length = n + (universe >> lower_bits) + 2;
        let mask = (1_u64 << lower_bits) - 1;
        let lower_bits_offset = higher_bits_length;
        let bv_len = lower_bits_offset + n * (lower_bits as u64);
        let b = FixedBitSet::with_capacity(bv_len as usize);

        EliasFano {
            universe,
            n,
            lower_bits,
            higher_bits_length,
            mask,
            lower_bits_offset,
            bv_len,
            b,
            cur_value: 0,
            position: 0,
            high_bits_pos: 0,
        }
    }

    pub fn compress(&mut self, elems: &[u64]) {
        let mut last = 0_u64;

        for (i, elem) in elems.iter().enumerate() {
            if i > 0 && *elem < last {
                panic!("Sequence is not sorted");
            }

            if *elem > self.universe {
                panic!("Element {} is greater than universe", elem);
            }

            let high = (*elem >> self.lower_bits) + i as u64 + 1;
            let low = elem & self.mask;

            self.b.set(high as usize, true);

            let offset = self.lower_bits_offset + (i as u64 * self.lower_bits);
            set_bits(&mut self.b, offset, low, self.lower_bits);

            last = *elem;

            if i == 0 {
                self.cur_value = *elem;
                self.high_bits_pos = high;
            }
        }
    }

    pub fn visit(&mut self, position: u64) -> Result<u64, OutOfBoundsError> {
        if position > self.size() {
            return Err(OutOfBoundsError);
        }

        if self.position == position {
            return Ok(self.value());
        }

        if position < self.position {
            self.reset();
        }

        let skip = position - self.position;
        let pos = (0..skip).fold(self.high_bits_pos, |pos, _| {
            get_next_set(&self.b, (pos + 1) as usize)
        });

        self.high_bits_pos = (pos - 1) as u64;
        self.position = position;
        self.read_current_value();
        Ok(self.value())
    }

    pub fn next(&mut self) -> Result<u64, OutOfBoundsError> {
        self.position += 1;

        if self.position >= self.size() {
            return Err(OutOfBoundsError);
        }

        self.read_current_value();
        Ok(self.value())
    }

    pub fn reset(&mut self) {
        self.high_bits_pos = 0;
        self.position = 0;
        self.read_current_value();
    }

    pub fn position(&self) -> u64 {
        self.position
    }

    pub fn value(&self) -> u64 {
        self.cur_value
    }

    pub fn bit_size(&self) -> usize {
        self.b.len()
    }

    pub fn size(&self) -> u64 {
        self.n
    }

    pub fn read_current_value(&mut self) {
        let pos = if self.high_bits_pos > 0 {
            self.high_bits_pos + 1
        } else {
            self.high_bits_pos
        };

        self.high_bits_pos = get_next_set(&self.b, pos as usize) as u64;

        let mut low = 0;
        let offset = self.lower_bits_offset + self.position * self.lower_bits;

        for i in 0..self.lower_bits {
            if self.b.contains((offset + i + 1) as usize) {
                low += 1;
            }
            low <<= 1;
        }
        low >>= 1;

        self.cur_value =
            (((self.high_bits_pos - self.position - 1) << self.lower_bits) | low) as u64;
    }
}

impl fmt::Display for EliasFano {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
    Universe: {:?}
    Elements: {:?}
    Lower_bits: {:?}
    Higher_bits_length: {:?}
    Mask: 0b{:?}
    Lower_bits_offset: {:?}
    Bitvector length: {:?}
",
            self.universe,
            self.n,
            self.lower_bits,
            self.higher_bits_length,
            self.mask,
            self.lower_bits_offset,
            self.bv_len,
        )
    }
}
