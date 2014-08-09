use super::token;
use super::tokenizer::TokenResult;

pub struct XPathTokenDisambiguator<T, I> {
    source: ::std::iter::Peekable<T, I>,
}

impl<T, I: Iterator<T>> XPathTokenDisambiguator<T, I> {
    pub fn new(source: I) -> XPathTokenDisambiguator<T, I> {
        XPathTokenDisambiguator{
            source: source.peekable(),
        }
    }
}

static node_test_names : [&'static str, .. 4] =
    [ "comment", "text", "processing-instruction", "node" ];

impl<I: Iterator<TokenResult>> Iterator<TokenResult> for XPathTokenDisambiguator<TokenResult, I> {
    fn next(&mut self) -> Option<TokenResult> {
        let token = self.source.next();
        let next  = self.source.peek();

        match (token, next) {
            (Some(Ok(token::String(val))), Some(&Ok(token::LeftParen))) => {
                if node_test_names.contains(&val.as_slice()) {
                    Some(Ok(token::NodeTest(val)))
                } else {
                    Some(Ok(token::Function(val)))
                }
            },
            (Some(Ok(token::String(val))), Some(&Ok(token::DoubleColon))) => {
                Some(Ok(token::Axis(val)))
            },
            (token, _) => token,
        }
    }
}
