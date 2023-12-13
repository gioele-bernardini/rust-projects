use std::collections::HashMap;
use car_park::park::Parking;

#[test]
fn test() {
    let mut hash_map: HashMap<String, String> = HashMap::new();

    hash_map.insert("CX106SP".to_string(), "James".to_string());
    hash_map.insert("SASSARI".to_string(), "Mario".to_string());

    let mut parking = Parking::new(hash_map, 3);
    assert_eq!(
        parking.park_car("ZZ121PS".to_string(), "Mario".to_string(), 10.),
        Ok(10. * 0.25)
    );

    assert_eq!(
        parking.park_car("RT534LL".to_string(), "Luca".to_string(), 10.),
        Err("No more spots available")
    );

    assert_eq!(
        parking.exit_parking("NO".to_string()),
        Err("Car not found")
    );
    
    assert_eq!(
        parking.exit_parking("ZZ121PS".to_string()),
        Ok(())
    );
}
