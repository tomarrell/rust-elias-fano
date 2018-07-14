extern crate elias_fano;

use elias_fano::EliasFano as EF;

fn main() {
    let mut thing = EF::new(2, 1);
    thing.info();
    thing = EF::new(100, 20);
    thing.info();
    thing = EF::new(0, 2);
    thing.info();
    thing = EF::new(291080, 12738992);
    thing.info();
}
