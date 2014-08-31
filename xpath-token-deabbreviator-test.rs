extern crate xpath;

use xpath::token;
use xpath::token::XPathToken;
use xpath::tokenizer::TokenResult;
use xpath::deabbreviator::XPathTokenDeabbreviator;

fn all_tokens_raw<I: Iterator<TokenResult>>(mut tokenizer: I) -> Result<Vec<XPathToken>, & 'static str> {
    tokenizer.collect()
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
    // let iter: &Iterator<TokenResult> = &input_tokens.move_iter();

    let deabbrv = XPathTokenDeabbreviator::new(input_tokens.move_iter());
    // let a: () = deabbrv.next();
    // println!("{}",a );

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
