use crate::utils::traverse::traverse;
use tree_sitter::{Node, Tree};

use super::traverse::TreeItem;

#[derive(Debug)]
pub struct Token<'a> {
    pub text: &'a str,
    pub kind: TokenKind,
    pub start_byte: usize,
    pub end_byte: usize,
}

impl Token<'_> {
    pub fn new(text: &'_ str, kind: TokenKind, start_byte: usize, end_byte: usize) -> Token<'_> {
        Token {
            text,
            kind,
            start_byte,
            end_byte,
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    NameFunction,
    NameClass,
    Name,
    Text,
}

pub fn tokenize_tree<'a>(tree: &'a Tree, source_code: &'a [u8]) -> impl Iterator<Item = Token<'a>> {
    let source = source_code;
    let tokens = traverse(tree.walk())
        .filter(|x| x.node.child_count() == 0)
        .map(|x| assign_context(&x, source));
    return tokens;
}

fn assign_context<'a>(item: &TreeItem, source: &'a [u8]) -> Token<'a> {
    let node = item.node;
    let field_name = item.field;

    let text = node.utf8_text(source).unwrap();

    if node.kind().contains("identifier") {
        match node.parent() {
            None => Token::new(text, TokenKind::Name, node.start_byte(), node.end_byte()),
            Some(parent) => {
                if is_class(&parent, field_name) {
                    Token::new(
                        text,
                        TokenKind::NameClass,
                        node.start_byte(),
                        node.end_byte(),
                    )
                } else if is_function(&parent, field_name) {
                    Token::new(
                        text,
                        TokenKind::NameFunction,
                        node.start_byte(),
                        node.end_byte(),
                    )
                } else {
                    match parent.parent() {
                        None => {
                            Token::new(text, TokenKind::Name, node.start_byte(), node.end_byte())
                        }
                        Some(gparent) => {
                            if is_class(&gparent, field_name) {
                                Token::new(
                                    text,
                                    TokenKind::NameClass,
                                    node.start_byte(),
                                    node.end_byte(),
                                )
                            } else {
                                Token::new(
                                    text,
                                    TokenKind::Name,
                                    node.start_byte(),
                                    node.end_byte(),
                                )
                            }
                        }
                    }
                }
            }
        }
    } else {
        Token::new(text, TokenKind::Text, node.start_byte(), node.end_byte())
    }
}

fn is_class(node: &Node, field_name: &str) -> bool {
    const CLASS_KW: [&'static str; 3] = ["class", "struct", "object"];

    CLASS_KW
        .iter()
        .any(|item| node.kind().contains(item) || field_name.contains(item))
}

fn is_function(node: &Node, field_name: &str) -> bool {
    const FUNCTION_KW: [&'static str; 4] = ["function", "method", "invocation", "call"];

    FUNCTION_KW
        .iter()
        .any(|item| node.kind().contains(item) || field_name.contains(item))
}
