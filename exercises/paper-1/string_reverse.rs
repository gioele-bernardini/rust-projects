fn string_reverse(s: &str) -> String {
    let mut out = String::new();

    for c in s.chars().rev() {
        out.push(c)
    }

    out
}

fn string_reverse_with_pop_match(s: &str) -> String {
    let mut out = String::new();
    let mut s = s.to_string();

    loop {
        match s.pop() {
            None => break,
            Some(c) => out.push(c),
        }
    }

    out
}

fn string_reverse_while_let(s: &str) -> String {
    let mut out = String::new();
    let mut s = s.to_string();

    while let Some(c) = s.pop() {
        out.push(c);
    }

    out
}

fn string_reverse_collect(s: &str) -> String {
    s.chars().rev().collect()
}
