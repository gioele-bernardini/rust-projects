use rand::Rng;

fn modify_odd(slice: &mut [u32]) {
    for val in slice.iter() {
        if val % 2 {
            val = 0;
        }
    }
}

fn main() {
    let seed = 42;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let size = 10;

    let random_array: Vec<u32> = (0..size)
        .map(|_| rng.gen_range(0..100))
        .collect();

    println!("{:?}", random_array);
}
