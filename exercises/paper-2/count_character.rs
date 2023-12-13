use std::collections::HashMap;

fn count_character(string: &str) -> HashMap<char, u32> {
    let mut map = HashMap::<char, u32>::new();

    for c in string.chars() {
        if let Some(val) = map.get_mut(&c) {
            *val += 1;
        } else {
            map.insert(c, 1);
        }
    }

    map
}

fn main() {
    let string = String::from("Ciao, mondo!");

    let map = count_character(&string);

    println!("{:?}", map);
}
