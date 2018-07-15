extern crate elias_fano;

use elias_fano::EliasFano;

#[test]
fn test_membership() {
    const NUM: u64 = 1000;
    let mut ef = EliasFano::new(NUM, NUM);
    let array: Vec<u64> = vec![0; NUM as usize]
        .iter()
        .enumerate()
        .map(|(idx, _)| { idx as u64 })
        .collect();

    ef.compress(&array);

    for (idx, v) in array.iter().enumerate() {
        // println!("{}", ef.value());

        if ef.value() != *v {
            panic!("{} is not the same as {}", ef.value(), v);
        }

        match ef.next() {
            Ok(_) => (),
            Err(_) => {
                if idx != array.len() - 1 {
                    panic!("Error returned when not at end of items");
                }
            }
        }
    }
}
