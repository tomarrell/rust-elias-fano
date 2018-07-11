extern crate fixedbitset;

use fixedbitset::FixedBitSet;

fn main() {
    let mut thing = EliasFano::new(2, 1);
    thing.info();
    thing = EliasFano::new(100, 20);
    thing.info();
    thing = EliasFano::new(0, 2);
    thing.info();
    thing = EliasFano::new(291080, 12738992);
    thing.info();
}

struct EliasFano {
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

impl EliasFano {
    fn new(universe: u64, n: u64) -> EliasFano {
        let lower_bits = if universe > n { msb(universe / n) } else { 0 };
        let higher_bits_length = n + (universe >> lower_bits) + 2;
        let mask = (1_u64 << lower_bits) - 1;
        let lower_bits_offset = higher_bits_length.clone();
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

    fn compress(&mut self, elems: &[u64]) {
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

            let offset = self.lower_bits_offset + i as u64 + self.lower_bits;
            set_bits(&mut self.b, offset, low, self.lower_bits);

            last = *elem;

            if i == 0 {
                self.cur_value = *elem;
                self.high_bits_pos = high;
            }
        }
    }

    fn value(&self) -> u64 {
        self.cur_value
    }

    fn size(&self) -> u64 {
        self.n
    }

    fn read_current_value(&mut self) {
        let mut pos = self.high_bits_pos;

        if pos > 0 {
            pos += 1
        }

        pos = get_next_set(&self.b, pos as usize);

        self.high_bits_pos = pos as u64;

        let mut low = 0;
        let offset = self.lower_bits_offset + self.position * self.lower_bits;

        for i in 0..self.lower_bits {
            if self.b.contains((offset + i + 1) as usize) {
                low += 1;
            }
            low = low << 1;
        }
        low = low << 1;

        self.cur_value = (self.high_bits_pos - self.position - 1) << self.lower_bits | low;
    }

    fn shift(&mut self) -> u64 {
        self.position += 1;

        if self.position >= self.size() {
            0
        } else {
            self.read_current_value();
            self.value()
        }
    }

    fn info(&self) {
        println!(
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
        );
    }
}

fn get_next_set(bitset: &FixedBitSet, index: usize) -> u64 {
    for i in index..bitset.len() {
        if bitset.contains(i) {
            return i as u64;
        }
    }
    0
}

fn set_bits(b: &mut FixedBitSet, offset: u64, bits: u64, length: u64) {
    for i in 0..length {
        let val = bits & (1 << (length - i - 1));
        b.set((offset + i + 1) as usize, val > 0);
    }
}

fn round(a: f64) -> u64 {
    if a < 0_f64 {
        (a - 0.5) as u64
    } else {
        (a + 0.5) as u64
    }
}

fn msb(x: u64) -> u64 {
    (round((x as f64).log2())) as u64
}
