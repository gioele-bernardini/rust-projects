fn lord_farquaad(s: String) -> String {
    s.replace("e", "X")
}

fn lord_farquaad_push(s: &String) -> String {
    let mut out = String::new();

    for c in s.chars() {
        if c == 'e' {
            out.push('X');
            continue;
        }
        out.push(c);
    }

    out
}

fn main() {
    let string1 = String::from("Ciao");

    let _string2 = lord_farquaad(string1);
    // println!("{} {}", string1, string2);     ERRORE OWNERSHIP

    let string3 = String::from("Ciao di nuovo! -> eee");
    let string4 = lord_farquaad_push(&string3);

    println!("{} {}", string3, string4);
}
