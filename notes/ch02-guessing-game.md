# Ch02 -- Programming a Guessing Game

Book: https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html
Code: [`exercises/ch02-guessing-game`](../exercises/ch02-guessing-game)

**Status:** in-progress

## One-line summary
Build the classic number-guessing game: read a line from stdin, parse it into a number, generate a random secret with an external crate, and `match` the comparison in a loop until the player wins.

## Key concepts
- `let` bindings are immutable by default; `mut` (as in `let mut guess`) opts a variable into being reassigned/mutated.
- `String::new()` creates an empty, growable `String`.
- `io::stdin().read_line(&mut guess)` reads a line of input and appends it to `guess` via a mutable reference (`&mut`), so no data is copied.
- `read_line` returns an `io::Result`; `.expect("message")` unwraps the `Ok` value or panics with that message on `Err`. Ignoring a `Result` produces a compiler warning -- Rust wants failure paths acknowledged.
- External crates are declared under `[dependencies]` in `Cargo.toml` (here, `rand = "0.10"`) and downloaded from crates.io; `Cargo.lock` pins the exact resolved versions so builds stay reproducible.
- `rand::random_range(1..=10)` returns a random number within an inclusive range.
- Shadowing: `let guess: u16 = ...` reuses the name `guess` for a new binding with a different type, converting the input `String` into a number without needing a second variable name.
- `.trim()` strips leading/trailing whitespace (including the newline `read_line` leaves in) before `.parse()` converts the string to a numeric type; `.parse()` returns a `Result`, so bad input can be matched instead of crashing.
- `match guess.trim().parse() { Ok(num) => num, Err(_) => continue }` is idiomatic recoverable error handling: invalid input just skips to the next loop iteration instead of panicking.
- `std::cmp::Ordering` is an enum with `Less`, `Greater`, `Equal` variants, returned by `.cmp()`; `match` on it must be exhaustive, which is why all three arms are handled.
- `loop { ... }` runs forever until an explicit `break`; here it re-prompts for guesses until the comparison hits `Ordering::Equal`.

## Quiz scores (retake until 100%)
- [x] Section 1: 2 / 2
- [x] Section 2: 1 / 1

## Things that confused me
- Why `read_line` needs `&mut guess` instead of just `guess`: passing a mutable reference lets the function write into the caller's `String` instead of taking (and having to hand back) ownership.
- `expect` vs. proper error handling: `.expect(...)` is fine for a learning exercise, but it crashes the whole program on bad input -- the later `match`/`continue` on `.parse()` is the more realistic pattern for input that's expected to sometimes be wrong.
- Shadowing `guess` (`String` -> `u16`) reads oddly at first coming from languages where a variable's type can't change, but it's just a fresh binding that happens to reuse the name.
- The test module (`mod tests`) is still the placeholder `assert_eq!(1 + 1, 2)` -- per the workflow in `main.rs`, this should be replaced with a real test (e.g. testing a small extracted comparison/parsing helper) before calling the chapter done.

## How this connects to earlier chapters
- Builds directly on ch01: still using `cargo run -p ch02-guessing-game` / `cargo test -p ch02-guessing-game` and `println!`, now inside a program that reads input and depends on an external crate rather than a static "hello world".
- First use of a `Cargo.toml` `[dependencies]` entry and `Cargo.lock` doing real work (pinning `rand`), rather than just being mentioned as a concept in ch01.
- First real use of `match` for control flow (on both `Result` and `Ordering`), which sets up pattern matching used more heavily in later chapters (ch06 enums/pattern matching, ch09 error handling).
