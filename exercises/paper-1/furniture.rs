use std::collections::HashMap;

fn get_furniture(furniture: &HashMap<String, f32>, name: String) -> f32 {
    furniture.get(name.as_str()).unwrap_or(&-1.0).clone()
    // trova il valore nella hashmap.. quindi bisogna clonarlo!
    // oppure -> &f32 (ritorno riferimento)
    // e tolgo .clone() 
}

fn get_furniture2(furniture: &HashMap<String, f32>, name: String) -> &f32 {
    match furniture.get(name.as_str()) {
        Some(x) => x,
        None => &1.0,
    }
}

fn get_furniture3(furniture: &HashMap<String, f32>, name: String) -> &f32 {
    if let Some(x) = furniture.get(name.as_str()) {
        &x
    } else {
        &-1.0
    }
}

fn main() {
    let mut furniture: HashMap<String, f32> = HashMap::new();

    furniture.insert("lavandino".to_string(), 100.0);
    furniture.insert("guardaroba".to_string(), 252.42);
    furniture.insert("televisione".to_string(), 415.23);

    let price: f32 = get_furniture(&furniture, "lavandino".to_string());
    println!("{}", price);
}
