use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is:{}", secret_number);
    loop {
        println!("Please Input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to Read Line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=> num,
            Err(_)=> continue,
        };

        println!("You guess {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
