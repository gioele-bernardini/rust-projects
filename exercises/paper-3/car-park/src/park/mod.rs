use std::collections::HashMap;

pub struct Parking {
    parked_cars: HashMap<String, String>,
    remaining_spots: u32,
}

impl Parking {
    pub fn new(parked_cars: HashMap<String, String>,
               max_capacity: u32) -> Self {

        let remaining_spots = max_capacity - parked_cars.len() as u32;

        Self {      // Cosa ritorna il costruttore
            parked_cars,
            remaining_spots,
        }
    }

    pub fn park_car(
        &mut self,
        car_plate: String,
        owner: String,
        minutes: f32,
    ) -> Result<f32, &str> {
        if self.remaining_spots > 0 {
            self.parked_cars.insert(car_plate, owner);
            self.remaining_spots -= 1;
            Ok(minutes * 0.25)
        } else {
            Err("No more spots available")
        }
    }

    pub fn exit_parking(&mut self, car_plate: String
                          ) -> Result<(), &str> {

        if let Some(_) = self.parked_cars.remove(&car_plate) {
            self.remaining_spots += 1;
            Ok(())
        } else {
            Err("Car not found")
        }
    }

    pub fn recognise_owner(s: String, hash: &mut HashMap<String, String>
                           ) -> Option<&String> {

        hash.get(&s)
    }
}
