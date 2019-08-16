fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
}

fn first_word(s: &String) -> &str {
    match s.as_bytes().iter().enumerate().find(|(_i, &x)| x == b' ') {
        None => &s[..],
        Some(pair) => &s[0..pair.0],
    }
}
