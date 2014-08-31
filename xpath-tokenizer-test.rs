extern crate xpath;

use xpath::token;
use xpath::token::XPathToken;
use xpath::tokenizer::XPathTokenizer;

fn is_finished(tokenizer: & XPathTokenizer) -> bool {
    ! tokenizer.has_more_tokens()
}

fn all_tokens_raw(mut tokenizer: XPathTokenizer) -> Result<Vec<XPathToken>, & 'static str> {
    tokenizer.collect()
}

fn all_tokens(tokenizer: XPathTokenizer) -> Vec<XPathToken> {
    match all_tokens_raw(tokenizer) {
        Ok(toks) => toks,
        Err(msg) => fail!(msg),
    }
}

#[test]
fn empty_string_has_no_tokens()
{
    let tokenizer = xpath::tokenizer::XPathTokenizer::new("");
    assert!(is_finished(&tokenizer));
}

#[test]
fn tokenizes_simple_string()
{
    let tokenizer = XPathTokenizer::new("hello");

    assert_eq!(all_tokens(tokenizer), vec!(token::String("hello".to_string())));
}

#[test]
fn tokenizes_grandchild_selector()
{
    let tokenizer = XPathTokenizer::new("hello/world");

    assert_eq!(all_tokens(tokenizer), vec!(token::String("hello".to_string()),
                                           token::Slash,
                                           token::String("world".to_string())));
}

#[test]
fn tokenizes_great_grandchild_selector()
{
    let tokenizer = XPathTokenizer::new("hello/there/world");

    assert_eq!(all_tokens(tokenizer), vec!(token::String("hello".to_string()),
                                           token::Slash,
                                           token::String("there".to_string()),
                                           token::Slash,
                                           token::String("world".to_string())));
}

#[test]
fn tokenizes_qualified_names()
{
    let tokenizer = XPathTokenizer::new("ns:foo");

    assert_eq!(all_tokens(tokenizer), vec!(token::PrefixedName("ns".to_string(), "foo".to_string())));
}

#[test]
fn ignores_whitespace_around_tokens()
{
    let tokenizer = XPathTokenizer::new(" @\t@\n@\r");

    assert_eq!(all_tokens(tokenizer), vec!(token::AtSign,
                                           token::AtSign,
                                           token::AtSign));
}

#[test]
fn tokenizes_wildcard_name_test()
{
    let tokenizer = XPathTokenizer::new("*");

    assert_eq!(all_tokens(tokenizer), vec!(token::String("*".to_string())));
}

#[test]
fn tokenizes_axis_separator()
{
    let tokenizer = XPathTokenizer::new("::");

    assert_eq!(all_tokens(tokenizer), vec!(token::DoubleColon));
}

#[test]
fn tokenizes_axis_selector()
{
    let tokenizer = XPathTokenizer::new("hello::world");

    assert_eq!(all_tokens(tokenizer), vec!(token::String("hello".to_string()),
                                           token::DoubleColon,
                                           token::String("world".to_string())));
}

#[test]
fn tokenizes_single_slash()
{
    let tokenizer = XPathTokenizer::new("/");

    assert_eq!(all_tokens(tokenizer), vec!(token::Slash));
}

#[test]
fn tokenizes_double_slash()
{
    let tokenizer = XPathTokenizer::new("//");

    assert_eq!(all_tokens(tokenizer), vec!(token::DoubleSlash));
}

#[test]
fn tokenizes_double_slash_separator()
{
    let tokenizer = XPathTokenizer::new("hello//world");

    assert_eq!(all_tokens(tokenizer), vec!(token::String("hello".to_string()),
                                           token::DoubleSlash,
                                           token::String("world".to_string())));
}

#[test]
fn tokenizes_left_paren()
{
    let tokenizer = XPathTokenizer::new("(");

    assert_eq!(all_tokens(tokenizer), vec!(token::LeftParen));
}

#[test]
fn tokenizes_right_paren()
{
    let tokenizer = XPathTokenizer::new(")");

    assert_eq!(all_tokens(tokenizer), vec!(token::RightParen));
}

#[test]
fn tokenizes_at_sign()
{
    let tokenizer = XPathTokenizer::new("@");

    assert_eq!(all_tokens(tokenizer), vec!(token::AtSign));
}

#[test]
fn tokenizes_single_dot()
{
    let tokenizer = XPathTokenizer::new(".");

    assert_eq!(all_tokens(tokenizer), vec!(token::CurrentNode));
}

#[test]
fn tokenizes_double_dot()
{
    let tokenizer = XPathTokenizer::new("..");

    assert_eq!(all_tokens(tokenizer), vec!(token::ParentNode));
}

#[test]
fn tokenizes_integral_number()
{
    let tokenizer = XPathTokenizer::new("42");

    assert_eq!(all_tokens(tokenizer), vec!(token::Number(42.0)));
}

#[test]
fn tokenizes_decimal_number()
{
    let tokenizer = XPathTokenizer::new("42.42");

    assert_eq!(all_tokens(tokenizer), vec!(token::Number(42.42)));
}

#[test]
fn tokenizes_decimal_number_without_integral_part()
{
    let tokenizer = XPathTokenizer::new(".40");

    assert_eq!(all_tokens(tokenizer), vec!(token::Number(0.40)));
}

#[test]
fn tokenizes_left_bracket()
{
    let tokenizer = XPathTokenizer::new("[");

    assert_eq!(all_tokens(tokenizer), vec!(token::LeftBracket));
}

#[test]
fn tokenizes_right_bracket()
{
    let tokenizer = XPathTokenizer::new("]");

    assert_eq!(all_tokens(tokenizer), vec!(token::RightBracket));
}

#[test]
fn tokenizes_apostrophe_literal()
{
    let tokenizer = XPathTokenizer::new("'hello!'");

    assert_eq!(all_tokens(tokenizer), vec!(token::Literal("hello!".to_string())));
}

#[test]
fn tokenizes_double_quote_literal()
{
    let tokenizer = XPathTokenizer::new("\"1.23\"");

    assert_eq!(all_tokens(tokenizer), vec!(token::Literal("1.23".to_string())));
}

#[test]
fn tokenizes_dollar_sign()
{
    let tokenizer = XPathTokenizer::new("$");

    assert_eq!(all_tokens(tokenizer), vec!(token::DollarSign));
}

#[test]
fn tokenizes_plus_sign()
{
    let tokenizer = XPathTokenizer::new("+");

    assert_eq!(all_tokens(tokenizer), vec!(token::PlusSign));
}

#[test]
fn tokenizes_minus_sign()
{
    let tokenizer = XPathTokenizer::new("-");

    assert_eq!(all_tokens(tokenizer), vec!(token::MinusSign));
}

#[test]
fn tokenizes_pipe()
{
    let tokenizer = XPathTokenizer::new("|");

    assert_eq!(all_tokens(tokenizer), vec!(token::Pipe));
}

#[test]
fn tokenizes_equal_sign()
{
    let tokenizer = XPathTokenizer::new("=");

    assert_eq!(all_tokens(tokenizer), vec!(token::Equal));
}

#[test]
fn tokenizes_not_equal_sign()
{
    let tokenizer = XPathTokenizer::new("!=");

    assert_eq!(all_tokens(tokenizer), vec!(token::NotEqual));
}

#[test]
fn tokenizes_less_than()
{
    let tokenizer = XPathTokenizer::new("<");

    assert_eq!(all_tokens(tokenizer), vec!(token::LessThan));
}

#[test]
fn tokenizes_less_than_or_equal()
{
    let tokenizer = XPathTokenizer::new("<=");

    assert_eq!(all_tokens(tokenizer), vec!(token::LessThanOrEqual));
}

#[test]
fn tokenizes_greater_than()
{
    let tokenizer = XPathTokenizer::new(">");

    assert_eq!(all_tokens(tokenizer), vec!(token::GreaterThan));
}

#[test]
fn tokenizes_greater_than_or_equal()
{
    let tokenizer = XPathTokenizer::new(">=");

    assert_eq!(all_tokens(tokenizer), vec!(token::GreaterThanOrEqual));
}

#[test]
fn special_preceding_token_forces_named_operator_and()
{
    let tokenizer = XPathTokenizer::new("1andz2");

    assert_eq!(all_tokens(tokenizer), vec!(token::Number(1.0),
                                           token::And,
                                           token::String("z2".to_string())));
}

#[test]
fn special_preceding_token_forces_named_operator_or()
{
    let tokenizer = XPathTokenizer::new("2oror");

    assert_eq!(all_tokens(tokenizer), vec!(token::Number(2.0),
                                           token::Or,
                                           token::String("or".to_string())));
}

#[test]
fn special_preceding_token_forces_named_operator_mod()
{
    let tokenizer = XPathTokenizer::new("3moddiv");

    assert_eq!(all_tokens(tokenizer), vec!(token::Number(3.0),
                                           token::Remainder,
                                           token::String("div".to_string())));
}

#[test]
fn special_preceding_token_forces_named_operator_div()
{
    let tokenizer = XPathTokenizer::new("1divz2");

    assert_eq!(all_tokens(tokenizer), vec!(token::Number(1.0),
                                           token::Divide,
                                           token::String("z2".to_string())));
}

#[test]
fn special_preceding_token_forces_named_operator_multiply()
{
    let tokenizer = XPathTokenizer::new("1*2");

    assert_eq!(all_tokens(tokenizer), vec!(token::Number(1.0),
                                           token::Multiply,
                                           token::Number(2.0)));
}

#[test]
fn exception_thrown_when_nothing_was_tokenized()
{
    let tokenizer = XPathTokenizer::new("!");
    let res = all_tokens_raw(tokenizer);

    assert!(res.is_err());
    assert!(res.unwrap_err().contains("create a token"));
}

#[test]
fn exception_thrown_when_name_test_has_no_local_name()
{
    let tokenizer = XPathTokenizer::new("ns:");
    let res = all_tokens_raw(tokenizer);

    assert!(res.is_err());
    assert!(res.unwrap_err().contains("missing a local name"));
}

#[test]
fn exception_thrown_when_quote_characters_mismatched()
{
    let tokenizer = XPathTokenizer::new("'hello\"");
    let res = all_tokens_raw(tokenizer);

    assert!(res.is_err());
    assert!(res.unwrap_err().contains("mismatched quote characters"));
}
