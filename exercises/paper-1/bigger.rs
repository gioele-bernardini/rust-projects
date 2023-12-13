fn bigger(x: i32, y: i32) -> i32 {
    if x >= y {
        return x;
    } else {
        return y;
    }
}

fn main() {
    let x = 10;
    let y = 15;

    let out = bigger(x, y);
    println!("{} is the bigger one!", out);
}
