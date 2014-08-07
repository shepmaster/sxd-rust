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
    xpath: Vec<char>,
    start: uint,
    prefer_recognition_of_operator_names: bool,
}

impl XPathTokenizer {
    pub fn new(xpath: & str) -> XPathTokenizer {
        XPathTokenizer {
            xpath: xpath.chars().collect(),
            start: 0,
            prefer_recognition_of_operator_names: false,
        }
    }

    pub fn has_more_tokens(& self) -> bool {
        self.xpath.len() > self.start
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

        if self.valid_ncname_start_char(offset) {
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

    fn tokenize_literal(& mut self, quote_char: char) -> XPathToken {
        let mut offset = self.start;

        offset += 1; // Skip over the starting quote
        let start_of_string = offset;

        offset = self.while_not_character(offset, quote_char);
        let end_of_string = offset;

        if self.xpath[offset] != quote_char {
            fail!("throw MismatchedQuoteCharacterException(quote_char)");
        }
        offset += 1; // Skip over ending quote

        self.start = offset;
        let value = self.xpath.slice(start_of_string, end_of_string);
        return Literal(String::from_chars(value));
    }

    fn substr(& self, start: uint, end: uint) -> String {
        String::from_chars(self.xpath.slice(start, end))
    }

    fn raw_next_token(& mut self) -> XPathToken {
        if self.xpath.len() >= self.start + 2 {
            let first_two = self.substr(self.start, self.start + 2);
            match self.two_char_tokens().find(&first_two) {
                Some(token) => {
                    self.start += 2;
                    return token.clone();
                }
                _ => {}
            }
        }

        let c = self.xpath[self.start];
        match self.single_char_tokens().find(&c) {
            Some(token) => {
                self.start += 1;
                return token.clone();
            }
            _ => {}
        }

        for quote_char in QUOTE_CHARS.iter() {
            if *quote_char == c {
                return self.tokenize_literal(*quote_char);
            }
        }

        if '.' == c {
            let has_more_chars = self.xpath.len() > self.start + 1;

            if ! has_more_chars ||
                has_more_chars && ! is_digit(self.xpath[self.start + 1]) {
                    // Ugly. Should we use START / FOLLOW constructs?
                    self.start += 1;
                    return CurrentNode;
                }
        }

        if is_number_char(c) {
            let mut offset = self.start;
            let current_start = self.start;

            offset = self.while_valid_number(offset);

            self.start = offset;
            let substr = self.substr(current_start, offset);
            match from_str(substr.as_slice()) {
                Some(value) => Number(value),
                None => fail!("Not really a number!")
            }
        } else {
            let mut offset = self.start;
            let current_start = self.start;

            if self.prefer_recognition_of_operator_names {
                for &(ref name, ref token) in self.named_operators().iter() {
                    let name_chars: Vec<char> = name.chars().collect();
                    let name_chars_slice = name_chars.as_slice();

                    if self.xpath.len() >= offset + name_chars.len() {
                        let xpath_chars = self.xpath.slice(offset, offset + name_chars.len());

                        if name_chars_slice == xpath_chars {
                            self.start += name_chars.len();
                            return token.clone();
                        }
                    }
                }
            }

            if self.xpath[offset] == '*' {
                self.start = offset + 1;
                return String("*".to_string());
            }

            offset = self.while_valid_string(offset);
            if self.xpath.len() >= offset + 2 &&
                self.xpath[offset] == ':' && self.xpath[offset + 1] != ':' {
                let prefix = self.substr(current_start, offset);

                offset += 1;

                let current_start = offset;
                offset = self.while_valid_string(offset);

                if current_start == offset {
                    fail!("throw MissingLocalNameException()");
                }

                let name = self.substr(current_start, offset);

                self.start = offset;
                return PrefixedName(prefix, name);
            } else {
                self.start = offset;
                return String(String::from_chars(self.xpath.slice(current_start, offset)));
            }
        }
    }

    fn consume_whitespace(& mut self) {
        while self.start < self.xpath.len() {
            let c = self.xpath[self.start];

            if ! is_xml_space(c) {
                break;
            }

            self.start += 1;
        }
    }

    pub fn next_token(& mut self) -> XPathToken {
        self.consume_whitespace();

        if ! self.has_more_tokens() {
            fail!("throw NoMoreTokensAvailableException()");
        }

        let old_start = self.start;
        let token = self.raw_next_token();

        if old_start == self.start {
            fail!("throw UnableToCreateTokenException()");
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
        return token;
    }
}

fn is_xml_space(c: char) -> bool {
    return
        c == ' '  ||
        c == '\t' ||
        c == '\n' ||
        c == '\r';
}

fn is_number_char(c: char) -> bool {
    return is_digit(c) || '.' == c;
}

// broken nesting double in single
// open paren / suqre brace causes indent
static QUOTE_CHARS: [char, .. 2] =  ['\'', '"'];
