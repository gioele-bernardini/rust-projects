use rand::Rng;

const MAX_ARRAY_LENGTH: u32  = 15;
const MAX_RANDOM_VALUE: u32 = 100;

fn modify_odd(slice: &mut [u32]) {
    for val in slice.iter_mut() {
        if *val % 2 == 1 {
            *val = 0;
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let array_length = rng.gen_range(1..=MAX_ARRAY_LENGTH);

    let mut random_array: Vec<_> = (0..array_length)
        .map(|_| rng.gen_range(1..=MAX_RANDOM_VALUE))
        .collect();

    println!("Random array:\n{:?}", random_array);

    modify_odd(&mut random_array);
    println!("Random array modified:\n{:?}", random_array);
}
