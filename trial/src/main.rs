use robotics_lib::energy::Energy;

fn energy_use(energy: &mut Energy) {
    let energy_needed = 10;
    if energy.has_enough_energy(energy_needed) {
       print!("I have enough energy to do the action");
    }
    else {
        print!("Oh no, I don't have enough energy 😴")}
}

fn main() {
    println!("Hello, world!");
}
