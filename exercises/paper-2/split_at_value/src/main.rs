use rand::Rng;

const MAX_ARRAY_LENGTH: usize = 15;
const MAX_RANDOM_VALUE: usize = 100;

fn split_at_value(slice: &[usize], target_value: usize) -> Option<(&[usize], &[usize])> {
    let mut target_index = Option::<usize>::None;

    for (i, val) in slice.iter().enumerate() {
        if *val == target_value {
            target_index = Option::Some(i);
            break;
        }
    }

    let target_index = target_index?;

    let s1 = &slice[0..target_index];
    let s2 = &slice[target_index..];

    Option::Some((s1, s2))
}

fn main() {
    let mut rng = rand::thread_rng();

    let array_length = rng.gen_range(1..=MAX_ARRAY_LENGTH);

    let random_array: Vec<_> = (0..array_length)
        .map(|_| rng.gen_range(1..=MAX_RANDOM_VALUE))
        .collect();

    println!("Random array:\n{:?}", random_array);

    // Estrai un valore casuale dall'array e copialo in una variabile
    let random_index = rng.gen_range(0..random_array.len());
    let random_value = random_array[random_index];
    
    println!("Random value extracted: {}", random_value);

    // Ora puoi chiamare la funzione split_at_value
    if let Some((s1, s2)) = split_at_value(&random_array[..], random_value) {
        println!("Random array slices:\n{:?}\n{:?}", s1, s2);
    } else {
        println!("Target value not found in the array.");
    }
}
