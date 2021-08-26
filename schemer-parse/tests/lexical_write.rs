use schemer_lang::read::tokens::*;
use schemer_lang::types::SchemeRepr;

#[test]
fn write_simple_comment() {
    let comment = Comment::from("hello");
    println!("{}", comment.to_repr_string());
}

#[test]
fn write_multiple_comment() {
    let mut comment = Comment::from("hello");
    comment.push_str("world!");
    println!("{}", comment.to_repr_string());
}

#[test]
fn write_nested_comment() {
    let mut comment = Comment::from("hello");
    comment.push_nested("world!".into());
    println!("{}", comment.to_repr_string());
}
