fn char_replace(s: String) -> String {
    s.replace("e", "X")
}

fn char_replace_push(s: String) -> String {
    let mut out = String::new();

    for c in s.chars() {
        if c == 'e' {
            out.push('X');
        } else {
            out.push(c);
        }
    }

    out
}
