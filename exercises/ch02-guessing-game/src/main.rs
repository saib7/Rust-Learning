// Chapter 02: Programming a Guessing Game
// Book: https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html
//
// Your first real program. Touches variables, input, match, Result, and crates all at once.
//
// Workflow:
//   1. Read the chapter (and do the in-browser quizzes -- they aren't
//      trackable in git, so just get them to 100% before moving on).
//   2. Take notes in notes/ch02-guessing-game.md
//   3. Replace this stub with real code: re-type the chapter's examples
//      from memory (don't copy-paste), then extend or add your own
//      small exercise that uses the chapter's concepts.
//   4. cargo run -p ch02-guessing-game   /   cargo test -p ch02-guessing-game
//   5. git add -A && git commit -m "ch02: Programming a Guessing Game"

use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::random_range(1..=10);
    // println!("The secret number is: {secret_number}");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");
        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

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

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        // Replace with a real test once you've written real code.
        assert_eq!(1 + 1, 2);
    }
}
