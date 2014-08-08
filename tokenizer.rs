use std::collections::hashmap::HashMap;
use std::char::is_digit;

#[deriving(PartialEq,Show,Clone)]
pub enum XPathToken {
    And,
    AtSign,
    CurrentNode,
    Divide,
    DollarSign,
    DoubleColon,
    DoubleSlash,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LeftBracket,
    LeftParen,
    LessThan,
    LessThanOrEqual,
    Literal(String),
    MinusSign,
    Multiply,
    NotEqual,
    Number(f64),
    Or,
    ParentNode,
    Pipe,
    PlusSign,
    PrefixedName(String, String),
    Remainder,
    RightBracket,
    RightParen,
    Slash,
    String(String),
}

impl XPathToken {
    fn precedes_node_test(& self) -> bool {
        match *self {
            AtSign |
            DoubleColon => true,
            _ => false,
        }
    }

    fn precedes_expression(& self) -> bool {
        match *self {
            LeftParen |
            LeftBracket => true,
            _ => false,
        }
    }

    fn is_operator(& self) -> bool {
        match *self {
            Slash |
            DoubleSlash |
            PlusSign |
            MinusSign |
            Pipe |
            Equal |
            NotEqual |
            LessThan |
            LessThanOrEqual |
            GreaterThan |
            GreaterThanOrEqual |
            And |
            Or |
            Remainder |
            Divide |
            Multiply => true,
            _ => false,
        }
    }
}

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
        m.insert("<=".to_string(), LessThanOrEqual);
        m.insert(">=".to_string(), GreaterThanOrEqual);
        m.insert("!=".to_string(), NotEqual);
        m.insert("::".to_string(), DoubleColon);
        m.insert("//".to_string(), DoubleSlash);
        m.insert("..".to_string(), ParentNode);
        m
    }

    fn single_char_tokens(&self) -> HashMap<char, XPathToken> {
        let mut m = HashMap::new();
        m.insert('/', Slash);
        m.insert('(', LeftParen);
        m.insert(')', RightParen);
        m.insert('[', LeftBracket);
        m.insert(']', RightBracket);
        m.insert('@', AtSign);
        m.insert('$', DollarSign);
        m.insert('+', PlusSign);
        m.insert('-', MinusSign);
        m.insert('|', Pipe);
        m.insert('=', Equal);
        m.insert('<', LessThan);
        m.insert('>', GreaterThan);
        m
    }

    fn named_operators(& self) -> Vec<(& 'static str, XPathToken)> {
        vec!(("and", And),
             ("or",  Or),
             ("mod", Remainder),
             ("div", Divide),
             ("*",   Multiply))
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
        return Ok(Literal(self.xpath.substr(start_of_string, end_of_string)));
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
                return Ok(CurrentNode);
            }
        }

        if is_number_char(c) {
            let mut offset = self.start;
            let current_start = self.start;

            offset = self.xpath.while_valid_number(offset);

            self.start = offset;
            let substr = self.xpath.substr(current_start, offset);
            match from_str(substr.as_slice()) {
                Some(value) => Ok(Number(value)),
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
                return Ok(String("*".to_string()));
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
                return Ok(PrefixedName(prefix, name));

            } else {
                self.start = offset;
                return Ok(String(self.xpath.substr(current_start, offset)));
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
