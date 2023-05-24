use similar::{ChangeTag, TextDiff};

fn main() {
    let diff = TextDiff::from_lines(
        "Hello, world!\nThis is test line.\n",
        "Hello, world!\nThis is test line.\n My name is Bob.",
    );

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        println!("{}{}", sign, change)
    }
}