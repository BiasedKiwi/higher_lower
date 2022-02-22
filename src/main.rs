use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let to_guess: i8 = rng.gen_range(0..100); // generate the number to guess
    println!("What's Up! I'm going to guess a number between 0 and 100 and you're going to have to guess it. Ok?");

    let mut first_guess: bool = true;
    
    loop { // ask for input until the user guesses correctly.
        let guess = ask(first_guess);
        first_guess = false;
        check_if_numeric(&guess);
        let guess: i8 = to_int(&guess).try_into().unwrap();
        if guess == to_guess {
            println!("You guessed it! The number was {}", to_guess);
            break;
        } else if guess > to_guess {
            println!("Too high! Try again.");
        } else {
            println!("Too low! Try again.");
        }
    }
}

fn to_int(s: &String) -> i32 { // self explanatory name.
    s.parse().unwrap()
}

fn ask(first_time: bool) -> String { // ask the user for input
    let mut guess = String::new();

    if first_time {
        println!("Input the number I guessed below!");
    }

    io::stdin().read_line(&mut guess).expect("Error!"); // read input
    guess.pop(); // do this because for some reason Rust adds another character at the end of string (might be the escape character)
    return guess;
}

fn check_if_numeric(s: &String) -> bool { // check if all characters are numbers, panic if not.
    for c in s.chars() {
        if !c.is_numeric() {
            panic!("Invalid Characters! Please input a number");
        }
    }
    return true;
}
