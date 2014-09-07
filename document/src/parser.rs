use std::ascii::AsciiExt;
use std::num::from_str_radix;
use std::char::from_u32;

use super::{Document,Element,Text,Comment,ProcessingInstruction};

pub struct Parser;

struct ParsedElement<'a> {
    name: &'a str,
    attributes: Vec<ParsedAttribute<'a>>,
    children: Vec<ParsedChild<'a>>,
}

enum ParsedAttributeValue<'a> {
    ReferenceAttributeValue(ParsedReference<'a>),
    LiteralAttributeValue(&'a str),
}

struct ParsedAttribute<'a> {
    name: &'a str,
    values: Vec<ParsedAttributeValue<'a>>,
}

struct ParsedText<'a> {
    text: &'a str,
}

struct ParsedEntityRef<'a> {
    text: &'a str,
}

struct ParsedDecimalCharRef<'a> {
    text: &'a str,
}

struct ParsedHexCharRef<'a> {
    text: &'a str,
}

enum ParsedReference<'a> {
    EntityParsedReference(ParsedEntityRef<'a>),
    DecimalCharParsedReference(ParsedDecimalCharRef<'a>),
    HexCharParsedReference(ParsedHexCharRef<'a>),
}

struct ParsedComment<'a> {
    text: &'a str,
}

struct ParsedProcessingInstruction<'a> {
    target: &'a str,
    value: Option<&'a str>,
}

enum ParsedRootChild<'a> {
    CommentParsedRootChild(ParsedComment<'a>),
    PIParsedRootChild(ParsedProcessingInstruction<'a>),
    IgnoredParsedRootChild,
}

enum ParsedChild<'a> {
    ElementParsedChild(ParsedElement<'a>),
    TextParsedChild(ParsedText<'a>),
    ReferenceParsedChild(ParsedReference<'a>),
    CommentParsedChild(ParsedComment<'a>),
    PIParsedChild(ParsedProcessingInstruction<'a>),
}

macro_rules! try_parse(
    ($e:expr) => ({
        match $e {
            None => return None,
            Some(x) => x,
        }
    })
)

// Pattern: 0-or-1
macro_rules! optional_parse(
    ($parser:expr, $start:expr) => ({
        match $parser {
            None => (None, $start),
            Some((value, next)) => (Some(value), next),
        }
    })
)

// Pattern: alternate
macro_rules! alternate_parse(
    ($start:expr, {}) => ( None );
    ($start:expr, {
        [$parser:expr -> $transformer:expr],
        $([$parser_rest:expr -> $transformer_rest:expr],)*
    }) => (
        match $parser($start) {
            Some((val, next)) => Some(($transformer(val), next)),
            None => alternate_parse!($start, {$([$parser_rest -> $transformer_rest],)*}),
        }
    );
)

// Pattern: zero-or-more
macro_rules! parse_zero_or_more(
    ($start:expr, $parser:expr) => {{
        let mut items = Vec::new();

        let mut start = $start;
        loop {
            let (item, next_start) = match $parser(start) {
                Some(x) => x,
                None => break,
            };

            items.push(item);
            start = next_start;
        }

        Some((items, start))
    }};
)

impl Parser {
    pub fn new() -> Parser {
        Parser
    }

    fn parse_eq<'a>(&self, xml: &'a str) -> Option<((), &'a str)> {
        let (_, xml) = optional_parse!(xml.slice_space(), xml);
        let (_, xml) = try_parse!(xml.slice_literal("="));
        let (_, xml) = optional_parse!(xml.slice_space(), xml);

        Some(((), xml))
    }

    fn parse_version_info<'a>(&self, xml: &'a str) -> Option<(&'a str, &'a str)> {
        let (_, xml) = try_parse!(xml.slice_space());
        let (_, xml) = try_parse!(xml.slice_literal("version"));
        let (_, xml) = try_parse!(self.parse_eq(xml));
        let (version, xml) = try_parse!(
            self.parse_quoted_value(xml, |xml, _| xml.slice_version_num())
        );

        Some((version, xml))
    }

    fn parse_xml_declaration<'a>(&self, xml: &'a str) -> Option<((), &'a str)> {
        let (_, xml) = try_parse!(xml.slice_literal("<?xml"));
        let (_version, xml) = try_parse!(self.parse_version_info(xml));
        // let (encoding, xml) = optional_parse!(self.parse_encoding_declaration(xml));
        // let (standalone, xml) = optional_parse!(self.parse_standalone_declaration(xml));
        let (_, xml) = optional_parse!(xml.slice_space(), xml);
        let (_, xml) = try_parse!(xml.slice_literal("?>"));

        Some(((), xml))
    }

    fn parse_space<'a>(&self, xml: &'a str) -> Option<(&'a str, &'a str)> {
        xml.slice_space()
    }

    fn parse_misc<'a>(&self, xml: &'a str) -> Option<(ParsedRootChild<'a>, &'a str)> {
        alternate_parse!(xml, {
            [|xml| self.parse_comment(xml) -> |c| CommentParsedRootChild(c)],
            [|xml| self.parse_pi(xml)      -> |p| PIParsedRootChild(p)],
            [|xml| self.parse_space(xml)   -> |_| IgnoredParsedRootChild],
        })
    }

    fn parse_miscs<'a>(&self, xml: &'a str) -> Option<(Vec<ParsedRootChild<'a>>, &'a str)> {
        parse_zero_or_more!(xml, |xml| self.parse_misc(xml))
    }

    fn parse_prolog<'a>(&self, xml: &'a str) -> Option<(Vec<ParsedRootChild<'a>>, &'a str)> {
        let (_, xml) = optional_parse!(self.parse_xml_declaration(xml), xml);
        self.parse_miscs(xml)
    }

    fn parse_one_quoted_value<'a, T>(&self,
                   xml: &'a str,
                   quote: &str,
                   f: |&'a str| -> Option<(T, &'a str)>)
                   -> Option<(T, &'a str)>
    {
        let (_, xml) = try_parse!(xml.slice_literal(quote));
        let (value, xml) = try_parse!(f(xml));
        let (_, xml) = try_parse!(xml.slice_literal(quote));

        Some((value, xml))
    }

    fn parse_quoted_value<'a, T>(&self,
                  xml: &'a str,
                  f: |&'a str, &str| -> Option<(T, &'a str)>)
                  -> Option<(T, &'a str)>
    {
        alternate_parse!(xml, {
            [|xml| self.parse_one_quoted_value(xml, "'",  |xml| f(xml, "'"))  -> |v| v],
            [|xml| self.parse_one_quoted_value(xml, "\"", |xml| f(xml, "\"")) -> |v| v],
        })
    }

    fn parse_attribute_values<'a>(&self, xml: &'a str, quote: &str)
                                 -> Option<(Vec<ParsedAttributeValue<'a>>, &'a str)>
    {
        parse_zero_or_more!(xml, |xml|
            alternate_parse!(xml, {
                [|xml: &'a str| xml.slice_attribute(quote) -> |v| LiteralAttributeValue(v)],
                [|xml: &'a str| self.parse_reference(xml)  -> |e| ReferenceAttributeValue(e)],
            }))
    }

    fn parse_attribute<'a>(&self, xml: &'a str) -> Option<(ParsedAttribute<'a>, &'a str)> {
        let (_, xml) = try_parse!(xml.slice_space());

        let (name, xml) = try_parse!(xml.slice_name());

        let (_, xml) = try_parse!(self.parse_eq(xml));

        let (values, xml) = try_parse!(
            self.parse_quoted_value(xml, |xml, quote| self.parse_attribute_values(xml, quote))
        );

        Some((ParsedAttribute{name: name, values: values}, xml))
    }

    fn parse_attributes<'a>(&self, xml: &'a str) -> Option<(Vec<ParsedAttribute<'a>>, &'a str)> {
        parse_zero_or_more!(xml, |xml| self.parse_attribute(xml))
    }

    fn parse_empty_element<'a>(&self, xml: &'a str) -> Option<(ParsedElement<'a>, &'a str)> {
        let (_, xml) = try_parse!(xml.slice_literal("<"));
        let (name, xml) = try_parse!(xml.slice_name());
        let (attrs, xml) = try_parse!(self.parse_attributes(xml));
        let (_, xml) = optional_parse!(xml.slice_space(), xml);
        let (_, xml) = try_parse!(xml.slice_literal("/>"));

        Some((ParsedElement{name: name, attributes: attrs, children: Vec::new()}, xml))
    }

    fn parse_element_start<'a>(&self, xml: &'a str) -> Option<(ParsedElement<'a>, &'a str)> {
        let (_, xml) = try_parse!(xml.slice_literal("<"));
        let (name, xml) = try_parse!(xml.slice_name());
        let (attrs, xml) = try_parse!(self.parse_attributes(xml));
        let (_, xml) = optional_parse!(xml.slice_space(), xml);
        let (_, xml) = try_parse!(xml.slice_literal(">"));

        Some((ParsedElement{name: name, attributes: attrs, children: Vec::new()}, xml))
    }

    fn parse_element_end<'a>(&self, xml: &'a str) -> Option<(&'a str, &'a str)> {
        let (_, xml) = try_parse!(xml.slice_literal("</"));
        let (name, xml) = try_parse!(xml.slice_name());
        let (_, xml) = optional_parse!(xml.slice_space(), xml);
        let (_, xml) = try_parse!(xml.slice_literal(">"));
        Some((name, xml))
    }

    fn parse_char_data<'a>(&self, xml: &'a str) -> Option<(ParsedText<'a>, &'a str)> {
        let (text, xml) = try_parse!(xml.slice_char_data());

        Some((ParsedText{text: text}, xml))
    }

    fn parse_cdata<'a>(&self, xml: &'a str) -> Option<(ParsedText<'a>, &'a str)> {
        let (_, xml) = try_parse!(xml.slice_literal("<![CDATA["));
        let (text, xml) = try_parse!(xml.slice_cdata());
        let (_, xml) = try_parse!(xml.slice_literal("]]>"));

        Some((ParsedText{text: text}, xml))
    }

    fn parse_entity_ref<'a>(&self, xml: &'a str) -> Option<(ParsedEntityRef<'a>, &'a str)> {
        let (_, xml) = try_parse!(xml.slice_literal("&"));
        let (name, xml) = try_parse!(xml.slice_name());
        let (_, xml) = try_parse!(xml.slice_literal(";"));

        Some((ParsedEntityRef{text: name}, xml))
    }

    fn parse_decimal_char_ref<'a>(&self, xml: &'a str) -> Option<(ParsedDecimalCharRef<'a>, &'a str)> {
        let (_, xml) = try_parse!(xml.slice_literal("&#"));
        let (dec, xml) = try_parse!(xml.slice_decimal_chars());
        let (_, xml) = try_parse!(xml.slice_literal(";"));

        Some((ParsedDecimalCharRef{text: dec}, xml))
    }

    fn parse_hex_char_ref<'a>(&self, xml: &'a str) -> Option<(ParsedHexCharRef<'a>, &'a str)> {
        let (_, xml) = try_parse!(xml.slice_literal("&#x"));
        let (hex, xml) = try_parse!(xml.slice_hex_chars());
        let (_, xml) = try_parse!(xml.slice_literal(";"));

        Some((ParsedHexCharRef{text: hex}, xml))
    }

    fn parse_reference<'a>(&self, xml: &'a str) -> Option<(ParsedReference<'a>, &'a str)> {
        alternate_parse!(xml, {
            [|xml| self.parse_entity_ref(xml)       -> |e| EntityParsedReference(e)],
            [|xml| self.parse_decimal_char_ref(xml) -> |d| DecimalCharParsedReference(d)],
            [|xml| self.parse_hex_char_ref(xml)     -> |h| HexCharParsedReference(h)],
        })
    }

    fn parse_comment<'a>(&self, xml: &'a str) -> Option<(ParsedComment<'a>, &'a str)> {
        let (_, xml) = try_parse!(xml.slice_literal("<!--"));
        let (text, xml) = try_parse!(xml.slice_comment());
        let (_, xml) = try_parse!(xml.slice_literal("-->"));

        Some((ParsedComment{text: text}, xml))
    }

    fn parse_pi_value<'a>(&self, xml: &'a str) -> Option<(&'a str, &'a str)> {
        let (_, xml) = try_parse!(xml.slice_space());
        xml.slice_pi_value()
    }

    fn parse_pi<'a>(&self, xml: &'a str) -> Option<(ParsedProcessingInstruction<'a>, &'a str)> {
        let (_, xml) = try_parse!(xml.slice_literal("<?"));
        let (target, xml) = try_parse!(xml.slice_name());
        let (value, xml) = optional_parse!(self.parse_pi_value(xml), xml);
        let (_, xml) = try_parse!(xml.slice_literal("?>"));

        if target.eq_ignore_ascii_case("xml") {
            fail!("Can't use xml as a PI target");
        }

        Some((ParsedProcessingInstruction{target: target, value: value}, xml))
    }

    fn parse_content<'a>(&self, xml: &'a str) -> (Vec<ParsedChild<'a>>, &'a str) {
        let mut children = Vec::new();

        let (char_data, xml) = optional_parse!(self.parse_char_data(xml), xml);
        char_data.map(|c| children.push(TextParsedChild(c)));

        // Pattern: zero-or-more
        let mut start = xml;
        loop {
            let xxx = alternate_parse!(start, {
                [|xml| self.parse_element(xml)   -> |e| ElementParsedChild(e)],
                [|xml| self.parse_cdata(xml)     -> |t| TextParsedChild(t)],
                [|xml| self.parse_reference(xml) -> |r| ReferenceParsedChild(r)],
                [|xml| self.parse_comment(xml)   -> |c| CommentParsedChild(c)],
                [|xml| self.parse_pi(xml)        -> |p| PIParsedChild(p)],
            });

            let (child, after) = match xxx {
                Some(x) => x,
                None => return (children, start),
            };

            let (char_data, xml) = optional_parse!(self.parse_char_data(after), after);

            children.push(child);
            char_data.map(|c| children.push(TextParsedChild(c)));

            start = xml;
        }
    }

    fn parse_non_empty_element<'a>(&self, xml: &'a str) -> Option<(ParsedElement<'a>, &'a str)> {
        let (mut element, xml) = try_parse!(self.parse_element_start(xml));
        let (children, xml) = self.parse_content(xml);
        let (name, xml) = try_parse!(self.parse_element_end(xml));

        if element.name != name {
            fail!("tags do not match!");
        }

        element.children = children;

        Some((element, xml))
    }

    fn parse_element<'a>(&self, xml: &'a str) -> Option<(ParsedElement<'a>, &'a str)> {
        alternate_parse!(xml, {
            [|xml| self.parse_empty_element(xml)     -> |e| e],
            [|xml| self.parse_non_empty_element(xml) -> |e| e],
        })
    }

    fn hydrate_text(&self, doc: &Document, text_data: ParsedText) -> Text {
        doc.new_text(text_data.text.to_string())
    }

    fn hydrate_reference_raw(&self, ref_data: ParsedReference) -> String {
        match ref_data {
            DecimalCharParsedReference(d) => {
                let code: u32 = from_str_radix(d.text, 10).expect("Not valid decimal");
                let c: char = from_u32(code).expect("Not a valid codepoint");
                c.to_string()
            },
            HexCharParsedReference(h) => {
                let code: u32 = from_str_radix(h.text, 16).expect("Not valid hex");
                let c: char = from_u32(code).expect("Not a valid codepoint");
                c.to_string()
            },
            EntityParsedReference(e) => {
                match e.text {
                    "amp"  => "&",
                    "lt"   => "<",
                    "gt"   => ">",
                    "apos" => "'",
                    "quot" => "\"",
                    _      => fail!("unknown entity"),
                }.to_string()
            }
        }
    }

    fn hydrate_reference(&self, doc: &Document, ref_data: ParsedReference) -> Text {
        doc.new_text(self.hydrate_reference_raw(ref_data))
    }

    fn hydrate_comment(&self, doc: &Document, comment_data: ParsedComment) -> Comment {
        doc.new_comment(comment_data.text.to_string())
    }

    fn hydrate_pi(&self, doc: &Document, pi_data: ParsedProcessingInstruction) -> ProcessingInstruction {
        doc.new_processing_instruction(pi_data.target.to_string(), pi_data.value.map(|v| v.to_string()))
    }

    fn hydrate_element(&self, doc: &Document, element_data: ParsedElement) -> Element {
        let element = doc.new_element(element_data.name.to_string());

        for attr in element_data.attributes.move_iter() {
            let to_v_str = |v: ParsedAttributeValue| match v {
                LiteralAttributeValue(v) => v.to_string(),
                ReferenceAttributeValue(r) => self.hydrate_reference_raw(r),
            };

            let v = attr.values.move_iter().fold(String::new(), |s, v| s.append(to_v_str(v).as_slice()));
            element.set_attribute(attr.name.to_string(), v);
        }

        for child in element_data.children.move_iter() {
            match child {
                ElementParsedChild(e)   => element.append_child(self.hydrate_element(doc, e)),
                TextParsedChild(t)      => element.append_child(self.hydrate_text(doc, t)),
                ReferenceParsedChild(r) => element.append_child(self.hydrate_reference(doc, r)),
                CommentParsedChild(c)   => element.append_child(self.hydrate_comment(doc, c)),
                PIParsedChild(pi)       => element.append_child(self.hydrate_pi(doc, pi)),
            }
        }

        element
    }

    fn hydrate_misc(&self, doc: &Document, children: Vec<ParsedRootChild>) {
        for child in children.move_iter() {
            match child {
                CommentParsedRootChild(c) =>
                    doc.root().append_child(self.hydrate_comment(doc, c)),
                PIParsedRootChild(p) =>
                    doc.root().append_child(self.hydrate_pi(doc, p)),
                IgnoredParsedRootChild => {},
            }
        }
    }

    fn hydrate_parsed_data(&self,
                           before_children: Vec<ParsedRootChild>,
                           element_data: ParsedElement,
                           after_children: Vec<ParsedRootChild>)
                           -> Document
    {
        let doc = Document::new();
        let root = doc.root();

        self.hydrate_misc(&doc, before_children);

        root.append_child(self.hydrate_element(&doc, element_data));

        self.hydrate_misc(&doc, after_children);

        doc
    }

    pub fn parse(&self, xml: &str) -> Document {
        let (before_children, xml) = optional_parse!(self.parse_prolog(xml), xml);
        let (element, xml) = self.parse_element(xml).expect("no element");
        let (after_children, _xml) = optional_parse!(self.parse_miscs(xml), xml);

        self.hydrate_parsed_data(before_children.unwrap_or(Vec::new()),
                                 element,
                                 after_children.unwrap_or(Vec::new()))
    }
}

trait XmlStr<'a> {
    fn slice_at(&self, position: uint) -> (&'a str, &'a str);
    fn slice_attribute(&self, quote: &str) -> Option<(&'a str, &'a str)>;
    fn slice_literal(&self, expected: &str) -> Option<(&'a str, &'a str)>;
    fn slice_version_num(&self) -> Option<(&'a str, &'a str)>;
    fn slice_char_data(&self) -> Option<(&'a str, &'a str)>;
    fn slice_cdata(&self) -> Option<(&'a str, &'a str)>;
    fn slice_decimal_chars(&self) -> Option<(&'a str, &'a str)>;
    fn slice_hex_chars(&self) -> Option<(&'a str, &'a str)>;
    fn slice_comment(&self) -> Option<(&'a str, &'a str)>;
    fn slice_pi_value(&self) -> Option<(&'a str, &'a str)>;
    fn slice_start_rest(&self, is_first: |char| -> bool, is_rest: |char| -> bool) -> Option<(&'a str, &'a str)>;
    fn slice_name(&self) -> Option<(&'a str, &'a str)>;
    fn slice_space(&self) -> Option<(&'a str, &'a str)>;
}

impl<'a> XmlStr<'a> for &'a str {
    fn slice_at(&self, position: uint) -> (&'a str, &'a str) {
        (self.slice_to(position), self.slice_from(position))
    }

    fn slice_attribute(&self, quote: &str) -> Option<(&'a str, &'a str)> {
        if self.starts_with("&") ||
           self.starts_with("<") ||
           self.starts_with(quote)
        {
            return None;
        }

        let (quote_char, _) = quote.slice_shift_char();
        let quote_char = quote_char.expect("Cant have null quote");

        let mut positions = self.char_indices().skip_while(|&(_, c)| c != '&' && c != '<' && c != quote_char);

        match positions.next() {
            Some((offset, _)) => Some(self.slice_at(offset)),
            None => Some((self.clone(), ""))
        }
    }

    fn slice_literal(&self, expected: &str) -> Option<(&'a str, &'a str)> {
        if self.starts_with(expected) {
            Some(self.slice_at(expected.len()))
        } else {
            None
        }
    }

    fn slice_version_num(&self) -> Option<(&'a str, &'a str)> {
        if self.starts_with("1.") {
            let mut positions = self.char_indices().peekable();
            positions.next();
            positions.next();

            // Need at least one character
            match positions.peek() {
                Some(&(_, c)) if c.is_decimal_char() => {},
                _ => return None,
            };

            let mut positions = positions.skip_while(|&(_, c)| c.is_decimal_char());
            match positions.next() {
                Some((offset, _)) => Some(self.slice_at(offset)),
                None => Some((self.clone(), "")),
            }
        } else {
            None
        }
    }


    fn slice_char_data(&self) -> Option<(&'a str, &'a str)> {
        if self.starts_with("<") ||
           self.starts_with("&") ||
           self.starts_with("]]>")
        {
            return None
        }

        // Using a hex literal because emacs' rust-mode doesn't
        // understand ] in a char literal. :-(
        let mut positions = self.char_indices().skip_while(|&(_, c)| c != '<' && c != '&' && c != '\x5d');

        loop {
            match positions.next() {
                None => return Some((self.clone(), "")),
                Some((offset, c)) if c == '<' || c == '&' => return Some(self.slice_at(offset)),
                Some((offset, _)) => {
                    let (head, tail) = self.slice_at(offset);
                    if tail.starts_with("]]>") {
                        return Some((head, tail))
                    } else {
                        // False alarm, resume scanning
                        continue;
                    }
                },
            }
        }
    }

    fn slice_cdata(&self) -> Option<(&'a str, &'a str)> {
        match self.find_str("]]>") {
            None => None,
            Some(offset) => Some(self.slice_at(offset)),
        }
    }

    fn slice_decimal_chars(&self) -> Option<(&'a str, &'a str)> {
        self.slice_start_rest(|c| c.is_decimal_char(),
                              |c| c.is_decimal_char())
    }

    fn slice_hex_chars(&self) -> Option<(&'a str, &'a str)> {
        self.slice_start_rest(|c| c.is_hex_char(),
                              |c| c.is_hex_char())
    }

    fn slice_comment(&self) -> Option<(&'a str, &'a str)> {
        // This deliberately does not include the >. -- is not allowed
        // in a comment, so we can just test the end if it matches the
        // complete close delimiter.
        match self.find_str("--") {
            None => None,
            Some(offset) => Some(self.slice_at(offset)),
        }
    }

    fn slice_pi_value(&self) -> Option<(&'a str, &'a str)> {
        match self.find_str("?>") {
            None => None,
            Some(offset) => Some(self.slice_at(offset)),
        }
    }

    fn slice_start_rest(&self,
                        is_first: |char| -> bool,
                        is_rest: |char| -> bool)
                        -> Option<(&'a str, &'a str)>
    {
        let mut positions = self.char_indices();

        match positions.next() {
            Some((_, c)) if is_first(c) => (),
            Some((_, _)) => return None,
            None => return None,
        };

        let mut positions = positions.skip_while(|&(_, c)| is_rest(c));
        match positions.next() {
            Some((offset, _)) => Some(self.slice_at(offset)),
            None => Some((self.clone(), "")),
        }
    }

    fn slice_name(&self) -> Option<(&'a str, &'a str)> {
        self.slice_start_rest(|c| c.is_name_start_char(), |c| c.is_name_char())
    }

    fn slice_space(&self) -> Option<(&'a str, &'a str)> {
        self.slice_start_rest(|c| c.is_space_char(), |c| c.is_space_char())
    }
}

trait XmlChar {
    fn is_name_start_char(&self) -> bool;
    fn is_name_char(&self) -> bool;
    fn is_space_char(&self) -> bool;
    fn is_decimal_char(&self) -> bool;
    fn is_hex_char(&self) -> bool;
}

impl XmlChar for char {
    fn is_name_start_char(&self) -> bool {
        match *self {
            ':'                        |
            'A'..'Z'                   |
            '_'                        |
            'a'..'z'                   |
            '\U000000C0'..'\U000000D6' |
            '\U000000D8'..'\U000000F6' |
            '\U000000F8'..'\U000002FF' |
            '\U00000370'..'\U0000037D' |
            '\U0000037F'..'\U00001FFF' |
            '\U0000200C'..'\U0000200D' |
            '\U00002070'..'\U0000218F' |
            '\U00002C00'..'\U00002FEF' |
            '\U00003001'..'\U0000D7FF' |
            '\U0000F900'..'\U0000FDCF' |
            '\U0000FDF0'..'\U0000FFFD' |
            '\U00010000'..'\U000EFFFF' => true,
            _ => false,
        }
    }

    fn is_name_char(&self) -> bool {
        if self.is_name_start_char() { return true; }
        match *self {
            '-'                |
            '.'                |
            '0'..'9'           |
            '\u00B7'           |
            '\u0300'..'\u036F' |
            '\u203F'..'\u2040' => true,
            _ => false
        }
    }

    fn is_space_char(&self) -> bool {
        match *self {
            '\x20' |
            '\x09' |
            '\x0D' |
            '\x0A' => true,
            _ => false,
        }
    }

    fn is_decimal_char(&self) -> bool {
        match *self {
            '0'..'9' => true,
            _ => false,
        }
    }

    fn is_hex_char(&self) -> bool {
        match *self {
            '0'..'9' |
            'a'..'f' |
            'A'..'F' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {

use super::Parser;

#[test]
fn a_document_with_a_prolog() {
    let parser = Parser::new();
    let doc = parser.parse("<?xml version='1.0' ?><hello />");
    let top = doc.root().children()[0].element().unwrap();

    assert_eq!(top.name().as_slice(), "hello");
}

#[test]
fn a_document_with_a_prolog_with_double_quotes() {
    let parser = Parser::new();
    let doc = parser.parse("<?xml version=\"1.0\" ?><hello />");
    let top = doc.root().children()[0].element().unwrap();

    assert_eq!(top.name().as_slice(), "hello");
}

#[test]
fn a_document_with_a_single_element() {
    let parser = Parser::new();
    let doc = parser.parse("<hello />");
    let top = doc.root().children()[0].element().unwrap();

    assert_eq!(top.name().as_slice(), "hello");
}

#[test]
fn an_element_with_an_attribute() {
    let parser = Parser::new();
    let doc = parser.parse("<hello scope='world'/>");
    let top = doc.root().children()[0].element().unwrap();

    assert_eq!(top.get_attribute("scope").unwrap().as_slice(), "world");
}

#[test]
fn an_element_with_an_attribute_using_double_quotes() {
    let parser = Parser::new();
    let doc = parser.parse("<hello scope=\"world\"/>");
    let top = doc.root().children()[0].element().unwrap();

    assert_eq!(top.get_attribute("scope").unwrap().as_slice(), "world");
}

#[test]
fn an_element_with_multiple_attributes() {
    let parser = Parser::new();
    let doc = parser.parse("<hello scope=\"world\" happy='true'/>");
    let top = doc.root().children()[0].element().unwrap();

    assert_eq!(top.get_attribute("scope").unwrap().as_slice(), "world");
    assert_eq!(top.get_attribute("happy").unwrap().as_slice(), "true");
}

#[test]
fn an_attribute_with_references() {
    let parser = Parser::new();
    let doc = parser.parse("<log msg='I &lt;3 math' />");
    let top = doc.root().children()[0].element().unwrap();

    assert_eq!(top.get_attribute("msg").unwrap().as_slice(), "I <3 math");
}

#[test]
fn an_element_that_is_not_self_closing() {
    let parser = Parser::new();
    let doc = parser.parse("<hello></hello>");
    let top = doc.root().children()[0].element().unwrap();

    assert_eq!(top.name().as_slice(), "hello");
}

#[test]
fn nested_elements() {
    let parser = Parser::new();
    let doc = parser.parse("<hello><world/></hello>");
    let nested = doc.root().children()[0].element().unwrap().children()[0].element().unwrap();

    assert_eq!(nested.name().as_slice(), "world");
}

#[test]
fn multiply_nested_elements() {
    let parser = Parser::new();
    let doc = parser.parse("<hello><awesome><world/></awesome></hello>");
    let hello = doc.root().children()[0].element().unwrap();
    let awesome = hello.children()[0].element().unwrap();
    let world = awesome.children()[0].element().unwrap();

    assert_eq!(world.name().as_slice(), "world");
}

#[test]
fn nested_elements_with_attributes() {
    let parser = Parser::new();
    let doc = parser.parse("<hello><world name='Earth'/></hello>");
    let hello = doc.root().children()[0].element().unwrap();
    let world = hello.children()[0].element().unwrap();

    assert_eq!(world.get_attribute("name").unwrap().as_slice(), "Earth");
}

#[test]
fn element_with_text() {
    let parser = Parser::new();
    let doc = parser.parse("<hello>world</hello>");
    let hello = doc.root().children()[0].element().unwrap();
    let text = hello.children()[0].text().unwrap();

    assert_eq!(text.text().as_slice(), "world");
}

#[test]
fn element_with_cdata() {
    let parser = Parser::new();
    let doc = parser.parse("<words><![CDATA[I have & and < !]]></words>");
    let words = doc.root().children()[0].element().unwrap();
    let text = words.children()[0].text().unwrap();

    assert_eq!(text.text().as_slice(), "I have & and < !");
}

#[test]
fn element_with_comment() {
    let parser = Parser::new();
    let doc = parser.parse("<hello><!-- A comment --></hello>");
    let words = doc.root().children()[0].element().unwrap();
    let comment = words.children()[0].comment().unwrap();

    assert_eq!(comment.text().as_slice(), " A comment ");
}

#[test]
fn comment_before_top_element() {
    let parser = Parser::new();
    let doc = parser.parse("<!-- A comment --><hello />");
    let comment = doc.root().children()[0].comment().unwrap();

    assert_eq!(comment.text().as_slice(), " A comment ");
}

#[test]
fn multiple_comments_before_top_element() {
    let parser = Parser::new();
    let xml = r"
<!--Comment 1-->
<!--Comment 2-->
<hello />";
    let doc = parser.parse(xml);
    let comment1 = doc.root().children()[0].comment().unwrap();
    let comment2 = doc.root().children()[1].comment().unwrap();

    assert_eq!(comment1.text().as_slice(), "Comment 1");
    assert_eq!(comment2.text().as_slice(), "Comment 2");
}

#[test]
fn multiple_comments_after_top_element() {
    let parser = Parser::new();
    let xml = r"
<hello />
<!--Comment 1-->
<!--Comment 2-->";
    let doc = parser.parse(xml);
    let comment1 = doc.root().children()[1].comment().unwrap();
    let comment2 = doc.root().children()[2].comment().unwrap();

    assert_eq!(comment1.text().as_slice(), "Comment 1");
    assert_eq!(comment2.text().as_slice(), "Comment 2");
}

#[test]
fn element_with_processing_instruction() {
    let parser = Parser::new();
    let doc = parser.parse("<hello><?device?></hello>");
    let hello = doc.root().children()[0].element().unwrap();
    let pi = hello.children()[0].processing_instruction().unwrap();

    assert_eq!(pi.target().as_slice(), "device");
    assert_eq!(pi.value(), None);
}

#[test]
fn top_level_processing_instructions() {
    let parser = Parser::new();
    let xml = r"
<?output printer?>
<hello />
<?validated?>";

    let doc = parser.parse(xml);
    let pi1 = doc.root().children()[0].processing_instruction().unwrap();
    let pi2 = doc.root().children()[2].processing_instruction().unwrap();

    assert_eq!(pi1.target().as_slice(), "output");
    assert_eq!(pi1.value().unwrap().as_slice(), "printer");

    assert_eq!(pi2.target().as_slice(), "validated");
    assert_eq!(pi2.value(), None);
}

#[test]
fn element_with_decimal_char_reference() {
    let parser = Parser::new();
    let doc = parser.parse("<math>2 &#62; 1</math>");
    let math = doc.root().children()[0].element().unwrap();
    let text1 = math.children()[0].text().unwrap();
    let text2 = math.children()[1].text().unwrap();
    let text3 = math.children()[2].text().unwrap();

    assert_eq!(text1.text().as_slice(), "2 ");
    assert_eq!(text2.text().as_slice(), ">");
    assert_eq!(text3.text().as_slice(), " 1");
}

#[test]
fn element_with_hexidecimal_char_reference() {
    let parser = Parser::new();
    let doc = parser.parse("<math>1 &#x3c; 2</math>");
    let math = doc.root().children()[0].element().unwrap();
    let text1 = math.children()[0].text().unwrap();
    let text2 = math.children()[1].text().unwrap();
    let text3 = math.children()[2].text().unwrap();

    assert_eq!(text1.text().as_slice(), "1 ");
    assert_eq!(text2.text().as_slice(), "<");
    assert_eq!(text3.text().as_slice(), " 2");
}

#[test]
fn element_with_entity_reference() {
    let parser = Parser::new();
    let doc = parser.parse("<math>I &lt;3 math</math>");
    let math = doc.root().children()[0].element().unwrap();
    let text1 = math.children()[0].text().unwrap();
    let text2 = math.children()[1].text().unwrap();
    let text3 = math.children()[2].text().unwrap();

    assert_eq!(text1.text().as_slice(), "I ");
    assert_eq!(text2.text().as_slice(), "<");
    assert_eq!(text3.text().as_slice(), "3 math");
}

#[test]
fn element_with_mixed_children() {
    let parser = Parser::new();
    let doc = parser.parse("<hello>to <a>the</a> world</hello>");
    let hello = doc.root().children()[0].element().unwrap();
    let text1 = hello.children()[0].text().unwrap();
    let middle = hello.children()[1].element().unwrap();
    let text2 = hello.children()[2].text().unwrap();

    assert_eq!(text1.text().as_slice(), "to ");
    assert_eq!(middle.name().as_slice(), "a");
    assert_eq!(text2.text().as_slice(), " world");
}

mod xmlstr {

use super::super::XmlStr;

#[test]
fn slice_char_data_leading_ampersand() {
    assert_eq!("&".slice_char_data(), None);
}

#[test]
fn slice_char_data_leading_less_than() {
    assert_eq!("<".slice_char_data(), None);
}

#[test]
fn slice_char_data_leading_cdata_end() {
    assert_eq!("]]>".slice_char_data(), None);
}

#[test]
fn slice_char_data_until_ampersand() {
    assert_eq!("hello&world".slice_char_data(), Some(("hello", "&world")));
}

#[test]
fn slice_char_data_until_less_than() {
    assert_eq!("hello<world".slice_char_data(), Some(("hello", "<world")));
}

#[test]
fn slice_char_data_until_cdata_end() {
    assert_eq!("hello]]>world".slice_char_data(), Some(("hello", "]]>world")));
}

#[test]
fn slice_char_data_includes_right_square() {
    assert_eq!("hello]world".slice_char_data(), Some(("hello]world", "")));
}

}
}
