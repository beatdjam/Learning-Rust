use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, world!");
    // 2. 数当てゲームをプログラムする
    // guessing_game();
    let s = String::from("helloworld");
    let word = first_word(&s);
    println!("{}", word);
}

fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessd: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn first_word(s: &String) -> &str {
    match s.as_bytes().iter().enumerate().find(|(_i, &x)| x == b' ') {
        None => &s[..],
        Some(pair) => &s[0..pair.0],
    }
}
