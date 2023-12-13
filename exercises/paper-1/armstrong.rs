fn armstrong(n: &i32) -> bool {
    let original = n;
    let mut n = *n;

    let mut digits = Vec::new();
    while n != 0 {
        digits.push( n % 10 );
        n /= 10;
    }

    let num_digits = digits.len() as u32;
    let mut sum = 0;
    for d in digits {
        sum += d.pow(num_digits);
    }

    sum == *original    
}

fn main() {
    let x = 153;

    println!("{}", armstrong(&x));
}
