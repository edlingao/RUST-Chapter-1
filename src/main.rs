use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn is_alive(lives: u32) -> bool {
    if lives == 0 {
        return false;
    }
    return true;
}


fn main() {
    println!("Guess the number");
    
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    let mut lives = 5;
    

    loop {
        let mut guess = String::new();
        print!("Please input your guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                lives -= 1;
                println!("Too small!");
            },
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You have {lives} lives left");
        if !is_alive(lives) {
            println!("You lose!");
            break;
        }
    }

}
