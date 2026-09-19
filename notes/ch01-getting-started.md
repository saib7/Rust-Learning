# Ch01 -- Getting Started

Book: https://rust-book.cs.brown.edu/ch01-00-getting-started.html
Code: [`exercises/ch01-getting-started`](../exercises/ch01-getting-started)

**Status:** in-progress

## One-line summary
Set up Rust, write a first `main` function, and use Cargo to create, build, run, and test a project.

## Key concepts
- `rustup` installs and manages Rust toolchains; `rustc --version` and `cargo --version` verify the installation.
- A Rust executable starts at `fn main()`.
- `println!` is a macro, identified by the `!`, and prints a line to standard output.
- `rustc` can compile a single Rust file, but Cargo is the standard tool for managing projects.
- `cargo new` creates a package with a `Cargo.toml` manifest and a `src/main.rs` entry point.
- `cargo run` compiles and runs a binary; `cargo build` compiles without running it; `cargo check` checks compilation without producing an executable.
- `cargo test` runs the tests in the package, and `cargo build --release` creates an optimized release build.
- `Cargo.lock` records resolved dependency versions so builds remain reproducible.

## Quiz scores (retake until 100%)
- [x] Section 1: 1/1
- [x] Section 2: 1/1

Scores are intentionally not guessed; record the browser quiz results here after completing both sections.

## Things that confused me
- The difference between `rustc` and Cargo: `rustc` is the compiler, while Cargo handles the project workflow around it.
- The exercise crate is still a starter program. Replace its placeholder output with a re-typed example and add a small test before marking this chapter done.

## How this connects to earlier chapters
- This is the first chapter, so there are no earlier Rust chapters to connect to. It establishes the toolchain and Cargo workflow used by every later exercise.
