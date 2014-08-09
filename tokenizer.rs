use std::collections::hashmap::HashMap;
use std::char::is_digit;

use super::token;
use super::token::XPathToken;

pub struct XPathTokenizer {
    xpath: XPathString,
    start: uint,
    prefer_recognition_of_operator_names: bool,
}

pub type TokenResult = Result<XPathToken, & 'static str>;

struct XPathString {
    xpath: Vec<char>,
}

impl XPathString {
    fn new(xpath: &str) -> XPathString {
        XPathString {
            xpath: xpath.chars().collect(),
        }
    }

    fn len(& self) -> uint {
        self.xpath.len()
    }

    fn str_at_is(& self, offset: uint, needle: &[char]) -> bool {
        let s_len = needle.len();

        if self.xpath.len() < offset + s_len { return false; }

        let xpath_chars = self.xpath.slice(offset, offset + s_len);

        needle == xpath_chars
    }


    fn valid_ncname_start_char(& self, offset: uint) -> bool {
        let c = self.xpath[offset];
        if c >= 'A' && c <= 'Z' { return true }
        if c == '_' { return true }
        if c >= 'a' && c <= 'z' { return true }
        // TODO: All non-ASCII codepoints
        return false;
    }

    fn valid_ncname_follow_char(& self, offset: uint) -> bool {
        let c = self.xpath[offset];
        if self.valid_ncname_start_char(offset) { return true }
        if c == '-' { return true }
        if c == '.' { return true }
        if c >= '0' && c <= '9' { return true }
        // TODO: All non-ASCII codepoints
        return false;
    }

    fn while_valid_string(& self, offset: uint) -> uint {
        let mut offset = offset;

        if offset < self.xpath.len() && self.valid_ncname_start_char(offset) {
            offset += 1;

            while offset < self.xpath.len() && self.valid_ncname_follow_char(offset) {
                offset += 1;
            }
        }

        return offset;
    }

    fn while_valid_number(& self, offset: uint) -> uint {
        let mut offset = offset;

        while offset < self.xpath.len() && is_number_char(self.xpath[offset]) {
            offset += 1;
        }

        return offset;
    }

    fn while_not_character(& self, offset: uint, end_char: char) -> uint {
        let mut offset = offset;

        while offset < self.xpath.len() && self.xpath[offset] != end_char {
            offset += 1;
        }

        return offset;
    }


    fn substr(& self, start: uint, end: uint) -> String {
        String::from_chars(self.xpath.slice(start, end))
    }

    fn safe_substr(& self, start: uint, end: uint) -> Option<String> {
        if self.xpath.len() >= end {
            Some(self.substr(start, end))
        } else {
            None
        }
    }

    fn char_at(&self, offset: uint) -> char {
        self.xpath[offset]
    }

    fn char_at_is(&self, offset: uint, c: char) -> bool {
        let has_one_more = self.xpath.len() >= offset + 1;

        has_one_more && self.xpath[offset] == c
    }

    fn char_at_is_not(&self, offset: uint, c: char) -> bool {
        let has_one_more = self.xpath.len() >= offset + 1;

        ! has_one_more || self.xpath[offset] != c
    }

    fn char_at_is_not_digit(& self, offset: uint) -> bool {
        let has_more_chars = self.xpath.len() >= offset + 1;

        ! has_more_chars || ! is_digit(self.xpath[offset])
    }

    fn is_xml_space(&self, offset: uint) -> bool {
        let c = self.xpath[offset];

        return
            c == ' '  ||
            c == '\t' ||
            c == '\n' ||
            c == '\r';
    }

    fn end_of_whitespace(& self, offset: uint) -> uint {
        let mut offset = offset;

        while offset < self.xpath.len() && self.is_xml_space(offset) {
            offset += 1;
        }

        offset
    }
}

static QUOTE_CHARS: [char, .. 2] =  ['\'', '\"'];

impl XPathTokenizer {
    pub fn new(xpath: & str) -> XPathTokenizer {
        XPathTokenizer {
            xpath: XPathString::new(xpath),
            start: 0,
            prefer_recognition_of_operator_names: false,
        }
    }

    pub fn has_more_tokens(& self) -> bool {
        self.xpath.len() > self.start
    }

    fn two_char_tokens(& self) -> HashMap<String, XPathToken> {
        let mut m = HashMap::new();
        m.insert("<=".to_string(), token::LessThanOrEqual);
        m.insert(">=".to_string(), token::GreaterThanOrEqual);
        m.insert("!=".to_string(), token::NotEqual);
        m.insert("::".to_string(), token::DoubleColon);
        m.insert("//".to_string(), token::DoubleSlash);
        m.insert("..".to_string(), token::ParentNode);
        m
    }

    fn single_char_tokens(&self) -> HashMap<char, XPathToken> {
        let mut m = HashMap::new();
        m.insert('/', token::Slash);
        m.insert('(', token::LeftParen);
        m.insert(')', token::RightParen);
        m.insert('[', token::LeftBracket);
        m.insert(']', token::RightBracket);
        m.insert('@', token::AtSign);
        m.insert('$', token::DollarSign);
        m.insert('+', token::PlusSign);
        m.insert('-', token::MinusSign);
        m.insert('|', token::Pipe);
        m.insert('=', token::Equal);
        m.insert('<', token::LessThan);
        m.insert('>', token::GreaterThan);
        m
    }

    fn named_operators(& self) -> Vec<(& 'static str, XPathToken)> {
        vec!(("and", token::And),
             ("or",  token::Or),
             ("mod", token::Remainder),
             ("div", token::Divide),
             ("*",   token::Multiply))
    }

    fn tokenize_literal(& mut self, quote_char: char) -> Result<XPathToken, & 'static str> {
        let mut offset = self.start;

        offset += 1; // Skip over the starting quote
        let start_of_string = offset;

        offset = self.xpath.while_not_character(offset, quote_char);
        let end_of_string = offset;

        if self.xpath.char_at_is_not(offset, quote_char) {
            return Err("found mismatched quote characters");
        }
        offset += 1; // Skip over ending quote

        self.start = offset;
        return Ok(token::Literal(self.xpath.substr(start_of_string, end_of_string)));
    }

    fn raw_next_token(& mut self) -> Result<XPathToken, & 'static str> {
        match self.xpath.safe_substr(self.start, self.start + 2) {
            Some(first_two) => {
                match self.two_char_tokens().find(&first_two) {
                    Some(token) => {
                        self.start += 2;
                        return Ok(token.clone());
                    }
                    _ => {}
                }
            },
            _ => {}
        }

        let c = self.xpath.char_at(self.start);

        match self.single_char_tokens().find(&c) {
            Some(token) => {
                self.start += 1;
                return Ok(token.clone());
            }
            _ => {}
        }

        for quote_char in QUOTE_CHARS.iter() {
            if *quote_char == c {
                return self.tokenize_literal(*quote_char);
            }
        }

        if '.' == c {
            if self.xpath.char_at_is_not_digit(self.start + 1) {
                // Ugly. Should we use START / FOLLOW constructs?
                self.start += 1;
                return Ok(token::CurrentNode);
            }
        }

        if is_number_char(c) {
            let mut offset = self.start;
            let current_start = self.start;

            offset = self.xpath.while_valid_number(offset);

            self.start = offset;
            let substr = self.xpath.substr(current_start, offset);
            match from_str(substr.as_slice()) {
                Some(value) => Ok(token::Number(value)),
                None => fail!("Not really a number!")
            }
        } else {
            let mut offset = self.start;
            let current_start = self.start;

            if self.prefer_recognition_of_operator_names {
                for &(ref name, ref token) in self.named_operators().iter() {
                    let name_chars: Vec<char> = name.chars().collect();
                    let name_chars_slice = name_chars.as_slice();

                    if self.xpath.str_at_is(offset, name_chars_slice) {
                        self.start += name_chars.len();
                        return Ok(token.clone());
                    }
                }
            }

            if self.xpath.char_at_is(offset, '*') {
                self.start = offset + 1;
                return Ok(token::String("*".to_string()));
            }

            offset = self.xpath.while_valid_string(offset);

            if self.xpath.char_at_is(offset, ':') && self.xpath.char_at_is_not(offset + 1, ':') {
                let prefix = self.xpath.substr(current_start, offset);

                offset += 1;

                let current_start = offset;
                offset = self.xpath.while_valid_string(offset);

                if current_start == offset {
                    return Err("The XPath is missing a local name");
                }

                let name = self.xpath.substr(current_start, offset);

                self.start = offset;
                return Ok(token::PrefixedName(prefix, name));

            } else {
                self.start = offset;
                return Ok(token::String(self.xpath.substr(current_start, offset)));
            }
        }
    }

    fn consume_whitespace(& mut self) {
        self.start = self.xpath.end_of_whitespace(self.start);
    }

    fn next_token(& mut self) -> TokenResult {
        self.consume_whitespace();

        let old_start = self.start;
        let token = self.raw_next_token();
        if token.is_err() { return token; }

        let token = token.unwrap();

        if old_start == self.start {
            return Err("Unable to create a token");
        }

        self.consume_whitespace();

        if ! (token.precedes_node_test() ||
              token.precedes_expression() ||
              token.is_operator()) {
            // See http://www.w3.org/TR/xpath/#exprlex
            self.prefer_recognition_of_operator_names = true;
        } else {
            self.prefer_recognition_of_operator_names = false;
        }

        return Ok(token);
    }
}

impl Iterator<TokenResult> for XPathTokenizer {
    fn next(&mut self) -> Option<TokenResult> {
        if self.has_more_tokens() {
            Some(self.next_token())
        } else {
            None
        }
    }
}

fn is_number_char(c: char) -> bool {
    return is_digit(c) || '.' == c;
}
