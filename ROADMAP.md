# Roadmap: fast-track Rust

A realistic "fast" pace is **4 weeks at ~1.5-2 focused hours/day, 5-6
days/week** (roughly 40-50 hours total). That's fast for a language with
a genuinely new mental model (ownership), not fast in the "watch a
3-hour video and know Rust" sense -- that version doesn't exist for this
language, and pretending it does is how people bounce off Rust twice.

If you have more or less time per day, keep the week groupings and
stretch or compress the day counts -- the grouping matters more than the
literal day count.

Check items off as you go; this file is meant to get edited and
committed like everything else.

## Week 1 -- Foundations + Ownership (Ch 1-6)

The single highest-leverage week. Ownership (Ch 4) is *the* Rust
concept -- everything later (lifetimes, smart pointers, concurrency
guarantees) is ownership applied to a new problem. Do not rush it, and
do not move to Week 2 until you can explain, out loud, why this doesn't
compile:

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{s1}"); // why does this fail?
```

- [ ] Ch01 -- Getting Started (fast; skim if you know another language)
- [ ] Ch02 -- Guessing Game (your first real program)
- [ ] Ch03 -- Common Programming Concepts (fast if experienced)
- [ ] Ch04 -- **Understanding Ownership** (slow down; use every Aquascope diagram; this is the one chapter worth reading twice)
- [ ] Ch05 -- Structs
- [ ] Ch06 -- Enums and Pattern Matching
- [ ] Checkpoint: explain ownership, borrowing, and the borrow checker to a rubber duck (or to me) without notes

## Week 2 -- Building real programs (Ch 7-12)

Where Rust stops being toy syntax and starts being how real codebases
are shaped. Chapter 12 is your first end-to-end project (`minigrep`) --
treat it as the payoff for Week 1, not just another chapter.

- [ ] Ch07 -- Packages, Crates, and Modules
- [ ] Ch08 -- Common Collections (Vec, String, HashMap)
- [ ] Ch09 -- Error Handling (`Result`, `?`, `panic!`)
- [ ] Ch10 -- Generics, Traits, and Lifetimes (second-hardest chapter; traits are everywhere from here on)
- [ ] Ch11 -- Writing Automated Tests
- [ ] Ch12 -- I/O Project: build `minigrep` end to end
- [ ] Checkpoint: `minigrep` builds, has tests, and you can extend it (e.g. add a `--count` flag) without looking anything up

## Week 3 -- Idiomatic Rust (Ch 13-16)

This is where code stops looking like "C with a borrow checker" and
starts looking like Rust: iterator chains instead of index loops, `Rc`/
`RefCell` for the cases ownership alone can't express cleanly, threads
that the compiler actually stops you from getting wrong.

- [ ] Ch13 -- Iterators and Closures
- [ ] Ch14 -- More about Cargo and Crates.io
- [ ] Ch15 -- Smart Pointers (Box, Rc, RefCell, Drop)
- [ ] Ch16 -- Fearless Concurrency (threads, channels, Mutex/Arc)
- [ ] Checkpoint: rewrite one of your Week 1-2 exercises using iterator adapters instead of manual loops

## Week 4 -- Advanced topics + capstone (Ch 17-21)

- [ ] Ch17 -- Async and Await
- [ ] Ch18 -- Object-Oriented Programming Features (trait objects, `dyn`)
- [ ] Ch19 -- Patterns and Matching (full reference; goes fast after Ch6)
- [ ] Ch20 -- Advanced Features (unsafe, advanced traits/types, macros -- read for awareness, don't expect fluency yet)
- [ ] Ch21 -- Final Project: multithreaded web server (build all three stages: single-threaded, thread-pooled, graceful shutdown)
- [ ] Checkpoint: the web server builds, serves a request, and shuts down cleanly on Ctrl+C

## After the book (ongoing)

The book gets you reading and writing Rust. Fluency comes from building
things with no chapter to lean on. Pick from `projects/README.md`:

- [ ] Project 1: a CLI tool you'd actually use
- [ ] Project 2: something involving `serde` + real-world data (JSON/CSV)
- [ ] Project 3: your choice -- pick something slightly too hard

Optional companion practice alongside any week above: [rustlings](https://github.com/rust-lang/rustlings) --
small broken programs you fix, good for drilling syntax between chapters
without the overhead of designing your own exercise from scratch.
