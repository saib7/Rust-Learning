// Chapter 11: Writing Automated Tests
// Book: https://rust-book.cs.brown.edu/ch11-00-testing.html
//
// #[test], assert! family, test organization. Start writing tests for everything after this.
//
// Workflow:
//   1. Read the chapter (and do the in-browser quizzes -- they aren't
//      trackable in git, so just get them to 100% before moving on).
//   2. Take notes in notes/ch11-testing.md
//   3. Replace this stub with real code: re-type the chapter's examples
//      from memory (don't copy-paste), then extend or add your own
//      small exercise that uses the chapter's concepts.
//   4. cargo run -p ch11-testing   /   cargo test -p ch11-testing
//   5. git add -A && git commit -m "ch11: Writing Automated Tests"

fn main() {
    println!("Chapter 11 -- Writing Automated Tests: not started yet");
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        // Replace with a real test once you've written real code.
        assert_eq!(1 + 1, 2);
    }
}
