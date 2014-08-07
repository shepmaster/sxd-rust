extern crate xpath;

use xpath::tokenizer::XPathToken;
use xpath::tokenizer::And;
use xpath::tokenizer::AtSign;
use xpath::tokenizer::CurrentNode;
use xpath::tokenizer::Divide;
use xpath::tokenizer::DollarSign;
use xpath::tokenizer::DoubleColon;
use xpath::tokenizer::DoubleSlash;
use xpath::tokenizer::Equal;
use xpath::tokenizer::GreaterThan;
use xpath::tokenizer::GreaterThanOrEqual;
use xpath::tokenizer::LeftBracket;
use xpath::tokenizer::LeftParen;
use xpath::tokenizer::LessThan;
use xpath::tokenizer::LessThanOrEqual;
use xpath::tokenizer::Literal;
use xpath::tokenizer::MinusSign;
use xpath::tokenizer::Multiply;
use xpath::tokenizer::NotEqual;
use xpath::tokenizer::Number;
use xpath::tokenizer::Or;
use xpath::tokenizer::ParentNode;
use xpath::tokenizer::Pipe;
use xpath::tokenizer::PlusSign;
use xpath::tokenizer::PrefixedName;
use xpath::tokenizer::Remainder;
use xpath::tokenizer::RightBracket;
use xpath::tokenizer::RightParen;
use xpath::tokenizer::Slash;
use xpath::tokenizer::String;
use xpath::tokenizer::XPathTokenizer;

fn is_finished(tokenizer: & XPathTokenizer) -> bool {
    ! tokenizer.has_more_tokens()
}

fn all_tokens(tokenizer: & mut XPathTokenizer) -> Vec<XPathToken> {
    let mut v = Vec::new();
    while tokenizer.has_more_tokens() {
        v.push(tokenizer.next_token());
    }
    v
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
    let mut tokenizer = XPathTokenizer::new("hello");

    assert_eq!(all_tokens(& mut tokenizer), vec!(String("hello".to_string())));
}

#[test]
fn tokenizes_grandchild_selector()
{
    let mut tokenizer = XPathTokenizer::new("hello/world");

    assert_eq!(all_tokens(& mut tokenizer), vec!(String("hello".to_string()),
                                           Slash,
                                           String("world".to_string())));
}

#[test]
fn tokenizes_great_grandchild_selector()
{
    let mut tokenizer = XPathTokenizer::new("hello/there/world");

    assert_eq!(all_tokens(& mut tokenizer), vec!(String("hello".to_string()),
                                           Slash,
                                           String("there".to_string()),
                                           Slash,
                                           String("world".to_string())));
}

#[test]
fn tokenizes_qualified_names()
{
    let mut tokenizer = XPathTokenizer::new("ns:foo");

    assert_eq!(all_tokens(& mut tokenizer), vec!(PrefixedName("ns".to_string(), "foo".to_string())));
}

#[test]
fn ignores_whitespace_around_tokens()
{
    let mut tokenizer = XPathTokenizer::new(" @\t@\n@\r");

    assert_eq!(all_tokens(& mut tokenizer), vec!(AtSign,
                                           AtSign,
                                           AtSign));
}

#[test]
fn tokenizes_wildcard_name_test()
{
    let mut tokenizer = XPathTokenizer::new("*");

    assert_eq!(all_tokens(& mut tokenizer), vec!(String("*".to_string())));
}

#[test]
fn tokenizes_axis_separator()
{
    let mut tokenizer = XPathTokenizer::new("::");

    assert_eq!(all_tokens(& mut tokenizer), vec!(DoubleColon));
}

#[test]
fn tokenizes_axis_selector()
{
    let mut tokenizer = XPathTokenizer::new("hello::world");

    assert_eq!(all_tokens(& mut tokenizer), vec!(String("hello".to_string()),
                                           DoubleColon,
                                           String("world".to_string())));
}

#[test]
fn tokenizes_single_slash()
{
    let mut tokenizer = XPathTokenizer::new("/");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Slash));
}

#[test]
fn tokenizes_double_slash()
{
    let mut tokenizer = XPathTokenizer::new("//");

    assert_eq!(all_tokens(& mut tokenizer), vec!(DoubleSlash));
}

#[test]
fn tokenizes_double_slash_separator()
{
    let mut tokenizer = XPathTokenizer::new("hello//world");

    assert_eq!(all_tokens(& mut tokenizer), vec!(String("hello".to_string()),
                                           DoubleSlash,
                                           String("world".to_string())));
}

#[test]
fn tokenizes_left_paren()
{
    let mut tokenizer = XPathTokenizer::new("(");

    assert_eq!(all_tokens(& mut tokenizer), vec!(LeftParen));
}

#[test]
fn tokenizes_right_paren()
{
    let mut tokenizer = XPathTokenizer::new(")");

    assert_eq!(all_tokens(& mut tokenizer), vec!(RightParen));
}

#[test]
fn tokenizes_at_sign()
{
    let mut tokenizer = XPathTokenizer::new("@");

    assert_eq!(all_tokens(& mut tokenizer), vec!(AtSign));
}

#[test]
fn tokenizes_single_dot()
{
    let mut tokenizer = XPathTokenizer::new(".");

    assert_eq!(all_tokens(& mut tokenizer), vec!(CurrentNode));
}

#[test]
fn tokenizes_double_dot()
{
    let mut tokenizer = XPathTokenizer::new("..");

    assert_eq!(all_tokens(& mut tokenizer), vec!(ParentNode));
}

#[test]
fn tokenizes_integral_number()
{
    let mut tokenizer = XPathTokenizer::new("42");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Number(42.0)));
}

#[test]
fn tokenizes_decimal_number()
{
    let mut tokenizer = XPathTokenizer::new("42.42");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Number(42.42)));
}

#[test]
fn tokenizes_decimal_number_without_integral_part()
{
    let mut tokenizer = XPathTokenizer::new(".40");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Number(0.40)));
}

#[test]
fn tokenizes_left_bracket()
{
    let mut tokenizer = XPathTokenizer::new("[");

    assert_eq!(all_tokens(& mut tokenizer), vec!(LeftBracket));
}

#[test]
fn tokenizes_right_bracket()
{
    let mut tokenizer = XPathTokenizer::new("]");

    assert_eq!(all_tokens(& mut tokenizer), vec!(RightBracket));
}

#[test]
fn tokenizes_apostrophe_literal()
{
    let mut tokenizer = XPathTokenizer::new("'hello!'");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Literal("hello!".to_string())));
}

#[test]
fn tokenizes_double_quote_literal()
{
    let mut tokenizer = XPathTokenizer::new("\"1.23\"");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Literal("1.23".to_string())));
}

#[test]
fn tokenizes_dollar_sign()
{
    let mut tokenizer = XPathTokenizer::new("$");

    assert_eq!(all_tokens(& mut tokenizer), vec!(DollarSign));
}

#[test]
fn tokenizes_plus_sign()
{
    let mut tokenizer = XPathTokenizer::new("+");

    assert_eq!(all_tokens(& mut tokenizer), vec!(PlusSign));
}

#[test]
fn tokenizes_minus_sign()
{
    let mut tokenizer = XPathTokenizer::new("-");

    assert_eq!(all_tokens(& mut tokenizer), vec!(MinusSign));
}

#[test]
fn tokenizes_pipe()
{
    let mut tokenizer = XPathTokenizer::new("|");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Pipe));
}

#[test]
fn tokenizes_equal_sign()
{
    let mut tokenizer = XPathTokenizer::new("=");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Equal));
}

#[test]
fn tokenizes_not_equal_sign()
{
    let mut tokenizer = XPathTokenizer::new("!=");

    assert_eq!(all_tokens(& mut tokenizer), vec!(NotEqual));
}

#[test]
fn tokenizes_less_than()
{
    let mut tokenizer = XPathTokenizer::new("<");

    assert_eq!(all_tokens(& mut tokenizer), vec!(LessThan));
}

#[test]
fn tokenizes_less_than_or_equal()
{
    let mut tokenizer = XPathTokenizer::new("<=");

    assert_eq!(all_tokens(& mut tokenizer), vec!(LessThanOrEqual));
}

#[test]
fn tokenizes_greater_than()
{
    let mut tokenizer = XPathTokenizer::new(">");

    assert_eq!(all_tokens(& mut tokenizer), vec!(GreaterThan));
}

#[test]
fn tokenizes_greater_than_or_equal()
{
    let mut tokenizer = XPathTokenizer::new(">=");

    assert_eq!(all_tokens(& mut tokenizer), vec!(GreaterThanOrEqual));
}

#[test]
fn special_preceding_token_forces_named_operator_and()
{
    let mut tokenizer = XPathTokenizer::new("1andz2");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Number(1.0),
                                           And,
                                           String("z2".to_string())));
}

#[test]
fn special_preceding_token_forces_named_operator_or()
{
    let mut tokenizer = XPathTokenizer::new("2oror");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Number(2.0),
                                           Or,
                                           String("or".to_string())));
}

#[test]
fn special_preceding_token_forces_named_operator_mod()
{
    let mut tokenizer = XPathTokenizer::new("3moddiv");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Number(3.0),
                                           Remainder,
                                           String("div".to_string())));
}

#[test]
fn special_preceding_token_forces_named_operator_div()
{
    let mut tokenizer = XPathTokenizer::new("1divz2");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Number(1.0),
                                           Divide,
                                           String("z2".to_string())));
}

#[test]
fn special_preceding_token_forces_named_operator_multiply()
{
    let mut tokenizer = XPathTokenizer::new("1*2");

    assert_eq!(all_tokens(& mut tokenizer), vec!(Number(1.0),
                                           Multiply,
                                           Number(2.0)));
}

#[test]
fn exception_thrown_when_no_more_tokens_available()
{
    fail!("unimplemented")
    // let mut tokenizer = XPathTokenizer::new("");

    // ASSERT_THROW(tokenizer.next_token(), NoMoreTokensAvailableException);
}

#[test]
fn exception_thrown_when_nothing_was_tokenized()
{
    fail!("unimplemented")
    //     let mut tokenizer = XPathTokenizer::new("!");

    // ASSERT_THROW(tokenizer.next_token(), UnableToCreateTokenException);
}

#[test]
fn exception_thrown_when_name_test_has_no_local_name()
{
    fail!("unimplemented")
    // let mut tokenizer = XPathTokenizer::new("ns:");

    // ASSERT_THROW(tokenizer.next_token(), MissingLocalNameException);
}

#[test]
fn exception_thrown_when_quote_characters_mismatched()
{
    fail!("unimplemented")
    // let mut tokenizer = XPathTokenizer::new("'hello\"");

    // ASSERT_THROW(tokenizer.next_token(), MismatchedQuoteCharacterException);
}
