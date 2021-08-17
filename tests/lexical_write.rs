use schemer::syntax::tokens::*;

#[test]
fn write_simple_comment() {
    let comment = Comment::from("hello");
    println!("{}", comment);
}

#[test]
fn write_multiple_comment() {
    let mut comment = Comment::from("hello");
    comment.push_str("world!");
    println!("{}", comment);
}

#[test]
fn write_nested_comment() {
    let mut comment = Comment::from("hello");
    comment.push_nested("world!".into());
    println!("{}", comment);
}
