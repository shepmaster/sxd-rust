#[cfg(test)]
mod tests {

impl XPathTokenizer {
    fn is_finished(& self) -> bool {
        ! self.has_more_tokens()
    }

    fn all_tokens(& self) -> Vec<XPathToken> {
        vec!()
    }
}

#[test]
fn empty_string_has_no_tokens()
{
    let tokenizer = XPathTokenizer::new("");
    assert!(tokenizer.is_finished());
}

#[test]
fn tokenizes_simple_string()
{
    let tokenizer = XPathTokenizer::new("hello");

    assert_eq!(tokenizer.all_tokens(), vec!(String("hello".to_string())));
}

#[test]
fn tokenizes_grandchild_selector()
{
    let tokenizer = XPathTokenizer::new("hello/world");

    assert_eq!(tokenizer.all_tokens(), vec!(String("hello".to_string()),
                                           Slash,
                                           String("world".to_string())));
}

#[test]
fn tokenizes_great_grandchild_selector()
{
    let tokenizer = XPathTokenizer::new("hello/there/world");

    assert_eq!(tokenizer.all_tokens(), vec!(String("hello".to_string()),
                                           Slash,
                                           String("there".to_string()),
                                           Slash,
                                           String("world".to_string())));
}

#[test]
fn tokenizes_qualified_names()
{
    let tokenizer = XPathTokenizer::new("ns:foo");

    assert_eq!(tokenizer.all_tokens(), vec!(PrefixedName("ns".to_string(), "foo".to_string())));
}

#[test]
fn ignores_whitespace_around_tokens()
{
    let tokenizer = XPathTokenizer::new(" @\t@\n@\r");

    assert_eq!(tokenizer.all_tokens(), vec!(AtSign,
                                           AtSign,
                                           AtSign));
}

#[test]
fn tokenizes_wildcard_name_test()
{
    let tokenizer = XPathTokenizer::new("*");

    assert_eq!(tokenizer.all_tokens(), vec!(String("*".to_string())));
}

#[test]
fn tokenizes_axis_separator()
{
    let tokenizer = XPathTokenizer::new("::");

    assert_eq!(tokenizer.all_tokens(), vec!(DoubleColon));
}

#[test]
fn tokenizes_axis_selector()
{
    let tokenizer = XPathTokenizer::new("hello::world");

    assert_eq!(tokenizer.all_tokens(), vec!(String("hello".to_string()),
                                           DoubleColon,
                                           String("world".to_string())));
}

#[test]
fn tokenizes_single_slash()
{
    let tokenizer = XPathTokenizer::new("/");

    assert_eq!(tokenizer.all_tokens(), vec!(Slash));
}

#[test]
fn tokenizes_double_slash()
{
    let tokenizer = XPathTokenizer::new("//");

    assert_eq!(tokenizer.all_tokens(), vec!(DoubleSlash));
}

#[test]
fn tokenizes_double_slash_separator()
{
    let tokenizer = XPathTokenizer::new("hello//world");

    assert_eq!(tokenizer.all_tokens(), vec!(String("hello".to_string()),
                                           DoubleSlash,
                                           String("world".to_string())));
}

#[test]
fn tokenizes_left_paren()
{
    let tokenizer = XPathTokenizer::new("(");

    assert_eq!(tokenizer.all_tokens(), vec!(LeftParen));
}

#[test]
fn tokenizes_right_paren()
{
    let tokenizer = XPathTokenizer::new(")");

    assert_eq!(tokenizer.all_tokens(), vec!(RightParen));
}

#[test]
fn tokenizes_at_sign()
{
    let tokenizer = XPathTokenizer::new("@");

    assert_eq!(tokenizer.all_tokens(), vec!(AtSign));
}

#[test]
fn tokenizes_single_dot()
{
    let tokenizer = XPathTokenizer::new(".");

    assert_eq!(tokenizer.all_tokens(), vec!(CurrentNode));
}

#[test]
fn tokenizes_double_dot()
{
    let tokenizer = XPathTokenizer::new("..");

    assert_eq!(tokenizer.all_tokens(), vec!(ParentNode));
}

#[test]
fn tokenizes_integral_number()
{
    let tokenizer = XPathTokenizer::new("42");

    assert_eq!(tokenizer.all_tokens(), vec!(Number(42.0)));
}

#[test]
fn tokenizes_decimal_number()
{
    let tokenizer = XPathTokenizer::new("42.42");

    assert_eq!(tokenizer.all_tokens(), vec!(Number(42.42)));
}

#[test]
fn tokenizes_decimal_number_without_integral_part()
{
    let tokenizer = XPathTokenizer::new(".42");

    assert_eq!(tokenizer.all_tokens(), vec!(Number(0.42)));
}

#[test]
fn tokenizes_left_bracket()
{
    let tokenizer = XPathTokenizer::new("[");

    assert_eq!(tokenizer.all_tokens(), vec!(LeftBracket));
}

#[test]
fn tokenizes_right_bracket()
{
    let tokenizer = XPathTokenizer::new("]");

    assert_eq!(tokenizer.all_tokens(), vec!(RightBracket));
}

#[test]
fn tokenizes_apostrophe_literal()
{
    let tokenizer = XPathTokenizer::new("'hello!'");

    assert_eq!(tokenizer.all_tokens(), vec!(Literal("hello!".to_string())));
}

#[test]
fn tokenizes_double_quote_literal()
{
    let tokenizer = XPathTokenizer::new("\"1.23\"");

    assert_eq!(tokenizer.all_tokens(), vec!(Literal("1.23".to_string())));
}

#[test]
fn tokenizes_dollar_sign()
{
    let tokenizer = XPathTokenizer::new("$");

    assert_eq!(tokenizer.all_tokens(), vec!(DollarSign));
}

#[test]
fn tokenizes_plus_sign()
{
    let tokenizer = XPathTokenizer::new("+");

    assert_eq!(tokenizer.all_tokens(), vec!(PlusSign));
}

#[test]
fn tokenizes_minus_sign()
{
    let tokenizer = XPathTokenizer::new("-");

    assert_eq!(tokenizer.all_tokens(), vec!(MinusSign));
}

#[test]
fn tokenizes_pipe()
{
    let tokenizer = XPathTokenizer::new("|");

    assert_eq!(tokenizer.all_tokens(), vec!(Pipe));
}

#[test]
fn tokenizes_equal_sign()
{
    let tokenizer = XPathTokenizer::new("=");

    assert_eq!(tokenizer.all_tokens(), vec!(Equal));
}

#[test]
fn tokenizes_not_equal_sign()
{
    let tokenizer = XPathTokenizer::new("!=");

    assert_eq!(tokenizer.all_tokens(), vec!(NotEqual));
}

#[test]
fn tokenizes_less_than()
{
    let tokenizer = XPathTokenizer::new("<");

    assert_eq!(tokenizer.all_tokens(), vec!(LessThan));
}

#[test]
fn tokenizes_less_than_or_equal()
{
    let tokenizer = XPathTokenizer::new("<=");

    assert_eq!(tokenizer.all_tokens(), vec!(LessThanOrEqual));
}

#[test]
fn tokenizes_greater_than()
{
    let tokenizer = XPathTokenizer::new(">");

    assert_eq!(tokenizer.all_tokens(), vec!(GreaterThan));
}

#[test]
fn tokenizes_greater_than_or_equal()
{
    let tokenizer = XPathTokenizer::new(">=");

    assert_eq!(tokenizer.all_tokens(), vec!(GreaterThanOrEqual));
}

#[test]
fn special_preceding_token_forces_named_operator_and()
{
    let tokenizer = XPathTokenizer::new("1andz2");

    assert_eq!(tokenizer.all_tokens(), vec!(Number(1.0),
                                           And,
                                           String("z2".to_string())));
}

#[test]
fn special_preceding_token_forces_named_operator_or()
{
    let tokenizer = XPathTokenizer::new("2oror");

    assert_eq!(tokenizer.all_tokens(), vec!(Number(2.0),
                                           Or,
                                           String("or".to_string())));
}

#[test]
fn special_preceding_token_forces_named_operator_mod()
{
    let tokenizer = XPathTokenizer::new("3moddiv");

    assert_eq!(tokenizer.all_tokens(), vec!(Number(3.0),
                                           Remainder,
                                           String("div".to_string())));
}

#[test]
fn special_preceding_token_forces_named_operator_div()
{
    let tokenizer = XPathTokenizer::new("1divz2");

    assert_eq!(tokenizer.all_tokens(), vec!(Number(1.0),
                                           Divide,
                                           String("z2".to_string())));
}

#[test]
fn special_preceding_token_forces_named_operator_multiply()
{
    let tokenizer = XPathTokenizer::new("1*2");

    assert_eq!(tokenizer.all_tokens(), vec!(Number(1.0),
                                           Multiply,
                                           Number(2.0)));
}

#[test]
fn exception_thrown_when_no_more_tokens_available()
{
    fail!("unimplemented")
    // let tokenizer = XPathTokenizer::new("");

    // ASSERT_THROW(tokenizer.next_token(), NoMoreTokensAvailableException);
}

#[test]
fn exception_thrown_when_nothing_was_tokenized()
{
    fail!("unimplemented")
    //     let tokenizer = XPathTokenizer::new("!");

    // ASSERT_THROW(tokenizer.next_token(), UnableToCreateTokenException);
}

#[test]
fn exception_thrown_when_name_test_has_no_local_name()
{
    fail!("unimplemented")
    // let tokenizer = XPathTokenizer::new("ns:");

    // ASSERT_THROW(tokenizer.next_token(), MissingLocalNameException);
}

#[test]
fn exception_thrown_when_quote_characters_mismatched()
{
    fail!("unimplemented")
    // let tokenizer = XPathTokenizer::new("'hello\"");

    // ASSERT_THROW(tokenizer.next_token(), MismatchedQuoteCharacterException);
}

}
