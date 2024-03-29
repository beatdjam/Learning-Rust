use rand::Rng;
use std::cmp::Ordering;
use std::io;

// 2. 数当てゲームをプログラムする
fn main() {
    guessing_game();
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