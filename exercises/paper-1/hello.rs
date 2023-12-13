fn double(x: i32) -> i32 {
    let mut x = x;
    x * 2
}

fn main() {
    let x = 10;
    let y = double(x);

    println!("{}", y);

    let a = 10;
    let b = a;

    println!("{} {}", a, b);

    let string1 = "ciao";
    let string2 = string1;

    println!("{}", string1);

    let string1 = String::from("ciao");
    let string2 = string1;

    println!("{}", string1);
}
