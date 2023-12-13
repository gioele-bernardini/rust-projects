fn max_min(v: &[i32]) -> (i32, i32) {
    if v.is_empty() {
        panic!("Il vettore è vuoto");
    }

    let mut max = v[0];
    let mut min = v[0];

    for &val in v.iter() {
        if val > max {
            max = val;
        }
        if val < min {
            min = val;
        }
    }

    (max, min)
}

fn max_min_recursive(v: &[i32], i: usize, j: usize, max: &mut i32, min: &mut i32) {
    if i >= j {
        return;
    }

    if v[i] > *max {
        *max = v[i];
    }
    if v[i] < *min {
        *min = v[i];
    }

    max_min_recursive(v, i + 1, j, max, min);
}

fn main() {
    let v = vec![10, 13, 35, 52, 12, 15, 62, 23, 97, 3];

    let out_tuple: (i32, i32) = max_min(&v);
    println!("{:?}", out_tuple);

    let mut max = 0;
    let mut min = 100;

    max_min_recursive(&v, 0, v.len(), &mut max, &mut min);
    println!("{} {}", max, min);
    println!("{:?}", v);
}
