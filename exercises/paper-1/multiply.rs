fn multiply(x: i32, y: f32, z: f64) -> f64 {
    return x as f64 * y as f64 * z;
}

fn main() {
    let x: i32 = 10;
    let y: f32 = 14.3;
    let z: f64 = 19.7;

    let out = multiply(x, y, z);
    println!("The result is {}", out);
}
