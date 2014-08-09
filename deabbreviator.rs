
use super::token;
use super::token::XPathToken;
use super::tokenizer::TokenResult;

pub struct XPathTokenDeabbreviator<I> {
    source: I,
    buffer: Vec<XPathToken>,
}

impl<I> XPathTokenDeabbreviator<I> {
    pub fn new(source: I) -> XPathTokenDeabbreviator<I> {
        XPathTokenDeabbreviator {
            source: source,
            buffer: vec!(),
        }
    }

    fn push(&mut self, token: XPathToken) {
        self.buffer.push(token);
    }

    fn expand_token(&mut self, token: XPathToken) {
        match token {
            token::AtSign => {
                self.push(token::String("attribute".to_string()));
                self.push(token::DoubleColon);
            }
            token::DoubleSlash => {
                self.push(token::Slash);
                self.push(token::String("descendant-or-self".to_string()));
                self.push(token::DoubleColon);
                self.push(token::String("node".to_string()));
                self.push(token::LeftParen);
                self.push(token::RightParen);
                self.push(token::Slash);
            }
            token::CurrentNode => {
                self.push(token::String("self".to_string()));
                self.push(token::DoubleColon);
                self.push(token::String("node".to_string()));
                self.push(token::LeftParen);
                self.push(token::RightParen);
            }
            token::ParentNode => {
                self.push(token::String("parent".to_string()));
                self.push(token::DoubleColon);
                self.push(token::String("node".to_string()));
                self.push(token::LeftParen);
                self.push(token::RightParen);
            }
            _ => {
                self.push(token);
            }
        }
    }
}

impl<I: Iterator<TokenResult>> Iterator<TokenResult> for XPathTokenDeabbreviator<I> {
    fn next(&mut self) -> Option<TokenResult> {
        if self.buffer.is_empty() {
            let token = self.source.next();

            match token {
                None => return token,
                Some(Err(_)) => return token,
                Some(Ok(token)) => self.expand_token(token),
            }
        }

        match self.buffer.remove(0) {
            Some(t) => Some(Ok(t)),
            None => fail!("No tokens left to return"), // Can't happen, we always add one
        }
    }
}
