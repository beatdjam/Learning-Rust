fn main() {
    let s = String::from("hello");
    let word = first_word(&s);
    println!("{}", word);
}
fn first_word(s: &String) -> &str {
    match s
        .as_bytes()
        .iter()
        .enumerate()
        .find(|(i, &x)| x == b' ' || i == &s.len())
    {
        None => &s[..],
        Some(pair) => &s[0..pair.0],
    }
}

//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(dimensions: &Rectangle) -> u32 {
//     dimensions.width * dimensions.height
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
