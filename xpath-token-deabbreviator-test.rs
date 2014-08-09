extern crate xpath;

use xpath::token;
use xpath::token::XPathToken;
use xpath::tokenizer::TokenResult;

struct XPathTokenDeabbreviator {
    a: int,
}

impl XPathTokenDeabbreviator {
    fn new<I: Iterator<TokenResult>>(source: I) -> XPathTokenDeabbreviator {
        XPathTokenDeabbreviator {a: 0}
    }
}

impl Iterator<TokenResult> for XPathTokenDeabbreviator {
    fn next(&mut self) -> Option<TokenResult> {
        None
    }
}

fn all_tokens_raw<I: Iterator<TokenResult>>(tokenizer: I) -> Result<Vec<XPathToken>, & 'static str> {
    std::result::collect(tokenizer)
}

fn all_tokens<I: Iterator<TokenResult>>(tokenizer: I) -> Vec<XPathToken> {
    match all_tokens_raw(tokenizer) {
        Ok(toks) => toks,
        Err(msg) => fail!(msg),
    }
}

#[test]
fn converts_at_sign_to_attribute_axis() {
    let input_tokens: Vec<TokenResult> = vec!(Ok(token::AtSign));

    let deabbrv = XPathTokenDeabbreviator::new(input_tokens.move_iter());

    assert_eq!(all_tokens(deabbrv), vec!(token::String("attribute".to_string()),
                                         token::DoubleColon));
}

#[test]
fn converts_double_slash_to_descendant_or_self() {
    let input_tokens: Vec<TokenResult> = vec!(Ok(token::DoubleSlash));

    let deabbrv = XPathTokenDeabbreviator::new(input_tokens.move_iter());

    assert_eq!(all_tokens(deabbrv), vec!(token::Slash,
                                         token::String("descendant-or-self".to_string()),
                                         token::DoubleColon,
                                         token::String("node".to_string()),
                                         token::LeftParen,
                                         token::RightParen,
                                         token::Slash));
}

#[test]
fn converts_current_node_to_self_node() {
    let input_tokens: Vec<TokenResult> = vec!(Ok(token::CurrentNode));

    let deabbrv = XPathTokenDeabbreviator::new(input_tokens.move_iter());

    assert_eq!(all_tokens(deabbrv), vec!(token::String("self".to_string()),
                                         token::DoubleColon,
                                         token::String("node".to_string()),
                                         token::LeftParen,
                                         token::RightParen));
}

#[test]
fn converts_parent_node_to_parent_node() {
    let input_tokens: Vec<TokenResult> = vec!(Ok(token::ParentNode));

    let deabbrv = XPathTokenDeabbreviator::new(input_tokens.move_iter());

    assert_eq!(all_tokens(deabbrv), vec!(token::String("parent".to_string()),
                                         token::DoubleColon,
                                         token::String("node".to_string()),
                                         token::LeftParen,
                                         token::RightParen));
}
