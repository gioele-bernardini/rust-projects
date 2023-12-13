fn append1(mut s: String) -> String {
    s.push_str("foobar");
    s
}

fn append2(s: String) -> String {
    let mut out = s.clone();
    let mut tmp = String::from("foobar");   // mutabile per il .pop()

    while let Some(x) = tmp.pop() {
        out.push(x);    // errato (esercizio 1 -> string_reverse(..))
    }

    out
}

fn append3(s: &String) -> String {
    let mut out = s.clone();
    out.push_str("foobar");
    out
}

fn append4(s: &mut String) {
    s.push_str("foobar");
}

fn main() {
    let string = String::from("Hello ");
    // let string2 = append(string1.clone());
    // let string2 = append(string1);

    let string1 = append1(string.clone());
    let string2 = append2(string.clone());
    let string3 = append3(&string.clone());
    let mut string4 = String::new("hello");
    append4(&mut string4);

    println!("{} {} {} {} {}", string, string1, string2,
             string3, string4);

}
