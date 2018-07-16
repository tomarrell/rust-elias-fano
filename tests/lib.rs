extern crate elias_fano;

use elias_fano::EliasFano;

#[test]
fn test_membership() {
    const NUM: u64 = 1000;
    let mut ef = EliasFano::new(NUM, NUM);
    let array: Vec<u64> = vec![0; NUM as usize]
        .iter()
        .enumerate()
        .map(|(idx, _)| idx as u64)
        .collect();

    ef.compress(&array);

    for (idx, v) in array.iter().enumerate() {
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

#[test]
fn test_position() {
    const NUM: u64 = 1000;
    let mut ef = EliasFano::new(NUM, NUM);
    let array: Vec<u64> = vec![0; NUM as usize]
        .iter()
        .enumerate()
        .map(|(idx, _)| idx as u64)
        .collect();

    ef.compress(&array);

    for i in 0..NUM {
        if ef.position() != i {
            panic!("Index is returning wrong position for linear increment")
        }
        ef.next();
    }
}

#[test]
fn test_reset() {
    const NUM: u64 = 1000;
    let mut ef = EliasFano::new(NUM, NUM);
    let array: Vec<u64> = vec![0; NUM as usize]
        .iter()
        .enumerate()
        .map(|(idx, _)| idx as u64)
        .collect();

    ef.compress(&array);

    if ef.position() != 0 {
        panic!("Initial position is not equal to 0");
    }

    ef.next();
    ef.reset();

    if ef.position() != 0 {
        panic!("Position was not reset correctly");
    }

    if ef.value() != 0 {
        panic!("Initial value is incorrect");
    }
}

#[test]
fn test_move() {
    const NUM: u64 = 1000;
    let mut ef = EliasFano::new(NUM, NUM);
    let array: Vec<u64> = vec![0; NUM as usize]
        .iter()
        .enumerate()
        .map(|(idx, _)| idx as u64)
        .collect();

    ef.compress(&array);

    if ef.position() != 0 {
        panic!("Initial position is not equal to 0");
    }

    for (idx, val) in array.iter().enumerate() {
        ef.visit(idx as u64);
        if ef.value() != *val {
            panic!("Received unexpected value after visit");
        }
    }

    for i in 0..NUM {
        ef.visit((array.len() - i as usize - 1) as u64);
        if ef.value() != array[array.len() - i as usize - 1] {
            panic!("Incorrect value found while visiting backwards");
        }
    }
}

#[test]
fn test_generic() {
    let mut ef = EliasFano::new(1000, 5);
    ef.compress(&[0, 5, 9, 800, 1000]);

    if ef.value() != 0 {
        panic!("Incorrect start value");
    }

    ef.visit(0);

    if ef.value() != 0 {
        panic!("0 visit returns different value");
    }

    ef.visit(4);

    if ef.value() != 1000 {
        panic!(
            "Visit returning incorrect value, expected: {}, received: {}",
            1000,
            ef.value()
        );
    }

    ef.reset();

    if ef.value() != 0 {
        panic!("Incorrect behaviour on reset");
    }

    ef.next();

    if ef.value() != 5 {
        panic!(
            "Next value is incorrect, expected: {}, received: {}",
            5,
            ef.value()
        );
    }

    ef.next();

    if ef.value() != 9 {
        panic!(
            "Next value is incorrect, expected: {}, received: {}",
            9,
            ef.value()
        );
    }

    ef.visit(1);

    if ef.value() != 5 {
        panic!(
            "Visit returning incorrect value, expected: {}, received: {}",
            5,
            ef.value()
        );
    }
}
