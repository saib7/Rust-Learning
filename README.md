# Learning Rust

My working repo for learning Rust with the [Brown University interactive
edition](https://rust-book.cs.brown.edu/) of *The Rust Programming
Language*. One Cargo crate per chapter, one notes file per chapter, real
git history so I (or anyone else working through the book) can see
exactly what was understood and when.

See [`ROADMAP.md`](ROADMAP.md) for the fast-track schedule.

## Layout

```
notes/          -- one markdown file per chapter: summary, quiz scores, confusions
exercises/      -- one Cargo crate per chapter (a workspace member each)
projects/       -- capstone projects built after finishing the book
Cargo.toml      -- workspace root; `cargo build --workspace` builds every chapter
.github/workflows/ci.yml -- builds + tests every chapter crate on push
```

Chapters follow the book's own numbering (1 through 21, including the
Async/Await and final multithreaded-web-server chapters added in the
book's more recent restructuring).

## Daily workflow

1. **Read** the chapter at `rust-book.cs.brown.edu`, and actually do the
   interactive quizzes in the browser -- retry until 100%. That's the one
   part of this system that can't live in git.
2. **Take notes** in `notes/chNN-slug.md` -- fill in the summary, quiz
   scores, and anything that confused you, *in your own words*. Writing
   it from memory after reading is the point; it's what makes the chapter
   stick instead of just having been seen.
3. **Write code** in `exercises/chNN-slug/src/main.rs`. Re-type the
   chapter's key examples from memory rather than copy-pasting, then
   extend them or do a small exercise of your own using that chapter's
   concepts. Copy-pasted code you didn't have to fight with teaches you
   almost nothing about ownership.
4. **Run it**: `cargo run -p chNN-slug` and `cargo test -p chNN-slug`.
5. **Commit**: `git add -A && git commit -m "chNN: <chapter title>"`.
   One commit per chapter (or per sitting) gives you a revision trail you
   can scroll back through later -- "what did I actually understand about
   lifetimes in week 2" is a `git log -p -- exercises/ch10-*` away.

Useful workspace-wide commands:

```bash
cargo build --workspace       # does everything still compile?
cargo test --workspace        # do all chapter tests still pass?
cargo run -p ch04-ownership   # run just one chapter's crate
```

## Putting this on GitHub

This repo is git-initialized locally already. To push it:

```bash
# create an empty repo on GitHub first (via github.com/new, no README/license), then:
git remote add origin https://github.com/<your-username>/rust-learning.git
git branch -M main
git push -u origin main
```

After that, every `git commit` you make locally, followed by `git push`,
keeps GitHub as your durable, shareable copy -- readable by you from any
machine, and by anyone else you point at the repo.

## Why this structure

- **One crate per chapter** means `cargo test --workspace` and the CI
  workflow catch it immediately if something you built in chapter 12
  breaks because of how you understood chapter 4 -- Rust's compiler is
  going to be your harshest and most useful teacher either way.
- **Notes are separate from code** because "I can write it" and "I can
  explain it" are different skills, and Rust's ownership/borrowing model
  is one of the few places in mainstream programming where you actually
  need both.
- **Git history is the point, not a formality.** The value of this repo
  a month from now isn't the final code, it's being able to see how your
  understanding of a concept evolved commit by commit.
