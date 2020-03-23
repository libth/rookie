use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");
    println!("input a number:");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a number! retry:");
                continue;
            },
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("**** guess good! ****");
                break;
            },
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
        }

        println!("guessed number: {}", guess);
    }
}
