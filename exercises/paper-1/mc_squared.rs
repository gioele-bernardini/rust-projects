const C: f32 = 299_792_458.0;

fn mc_squared(m: f32) -> f32 {
    m*C.powi(2)
}

fn main() {
    let mass: f32 = 15.0;

    let out = mc_squared(mass);
    println!("The result is {}", out);
}
