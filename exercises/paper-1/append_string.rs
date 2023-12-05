fn append_string(mut s: String) -> String {
    s.push_str("foobar");
    s
}

fn append_string2(s: &mut String) {
    s.push_str("foobar");
}
