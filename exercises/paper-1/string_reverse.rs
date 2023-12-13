fn string_reverse(s: &str) -> String {
    let mut out = String::new();

    for c in s.chars() {
        out.push(c);
    }

    out
}

fn main() {
    let s = "Hello, world!";
    let out = string_reverse(&s);

    println!("{}", out);
}
