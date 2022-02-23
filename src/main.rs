use std::io;
use rand::Rng;
 
/// ==================
/// == Main Program ==
/// ==================

fn main() {
    let mut rng = rand::thread_rng();
    let to_guess: i8 = rng.gen_range(0..100); // Generate the random number to guess
    println!("Hi! I'm going to guess a number between 0 and 100 and you're going to have to guess it. Ok?");

    let mut first_guess: bool = true;
    
    loop { // Ask for input until the user guesses correctly.
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

// =======================
// == Utility Functions ==
// =======================

/// Converts a String to a i32.
///
/// # Examples
///
/// ```
/// let s = String::from("123");
/// assert_eq!(to_int(&s), 123);
/// ```
fn to_int(s: &String) -> i32 {
    s.parse().unwrap()
}

/// Ask the user for input.
fn ask(first_time: bool) -> String {
    let mut guess = String::new();

    if first_time {
        println!("Input the number I guessed below!");
    }

    io::stdin().read_line(&mut guess).expect("Error!"); // read input
    guess.pop(); // Do this because for some reason Rust adds another character at the end of string (might be the escape character)
    return guess;
}

/// Check if all characters provided are numbers, panic if not.
/// 
/// # Panics
/// 
/// Panics if `s` contains non-numeric chars.
/// 
/// # Examples
/// 
/// ```
/// let s = String::from("123");
/// assert_eq!(check_if_numeric(&s), true);
/// 
/// let s = String::from("123a");
/// assert_eq!(check_if_numeric(&s), false); // panics with "Invalid Characters! Please only input numbers."
/// ```
fn check_if_numeric(s: &String) -> bool {
    for c in s.chars() {  // Loop through all the chars in the String.
        if !c.is_numeric() {
            panic!("Invalid Characters! Please only input numbers.");
        }
    }
    return true;
}

#[cfg(test)]
mod tests;
