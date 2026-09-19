# Beyond the book

Once you finish Chapter 21 (the multithreaded web server), the fastest way
to make Rust stick is to build something of your own with no chapter to
lean on. Put those projects here, each in its own `cargo new` crate,
added to the root workspace `Cargo.toml` `members` list the same way the
`exercises/*` glob already picks up everything under `exercises/`.

Good first "own projects" (roughly increasing difficulty):
- A CLI tool you'd actually use (a habit tracker, a unit converter, a
  markdown-to-html renderer) -- exercises `clap` for argument parsing,
  `std::fs`, error handling with `?`.
- A small JSON or CSV processor -- exercises `serde`, iterators, error
  handling on malformed input.
- A toy key-value store with a save-to-disk file format -- exercises
  ownership of longer-lived structures, `Result`, basic file I/O.
- A multi-client chat server extending Ch16/Ch21's concurrency and
  networking ideas.

One project you finish and can explain teaches more than five you start
and abandon. Pick one, finish it, write a short README for it, then move
to the next.
