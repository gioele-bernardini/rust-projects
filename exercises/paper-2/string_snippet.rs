fn main() {
    let mut string = String::from("Hello, world!");

    for s in string.chars() {
        s = 'X';
    }

    println!("{}", string);
}
