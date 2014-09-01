use std::iter::Peekable;

use super::{String,Number};
use super::token;
use super::token::XPathToken;
use super::tokenizer::TokenResult;
use super::expression::XPathExpression;
use super::expression::{
    ExpressionAnd,
    ExpressionEqual,
    ExpressionFunction,
    ExpressionLiteral,
    ExpressionMath,
    ExpressionNegation,
    ExpressionNotEqual,
    ExpressionOr,
    ExpressionRelational,
    ExpressionVariable,
};

pub struct XPathParser;

impl XPathParser {
    pub fn new() -> XPathParser {
        XPathParser
    }
}

pub type SubExpression = Box<XPathExpression + 'static>;

#[deriving(Show,PartialEq,Clone)]
pub enum ParseErr {
    RanOutOfInput,
    UnexpectedToken(token::XPathToken),
    RightHandSideExpressionMissing,
    ExtraUnparsedTokens,
    TokenizerError(&'static str),
}

pub type ParseResult = Result<Option<SubExpression>, ParseErr>;

type BinaryExpressionBuilder = fn(SubExpression, SubExpression) -> SubExpression;

struct BinaryRule {
    token: XPathToken,
    builder: BinaryExpressionBuilder,
}

struct LeftAssociativeBinaryParser<I> {
    rules: Vec<BinaryRule>,
}

type TokenSource<'a, I> = &'a mut Peekable<TokenResult, I>;

trait XCompat {
    fn has_more_tokens(&mut self) -> bool;
    fn next_token_is(&mut self, token: &XPathToken) -> bool;
    fn consume(&mut self, token: &XPathToken) -> Result<(), ParseErr>;
}

impl<I: Iterator<TokenResult>> XCompat for Peekable<TokenResult, I> {
    fn has_more_tokens(&mut self) -> bool {
        self.peek().is_some()
    }

    fn next_token_is(&mut self, token: &XPathToken) -> bool {
        match self.peek() {
            Some(&Ok(ref t)) => t == token,
            _ => false
        }
    }

    fn consume(&mut self, token: &XPathToken) -> Result<(), ParseErr> {
        match self.next() {
            None => Err(RanOutOfInput),
            Some(Err(x)) => Err(TokenizerError(x)),
            Some(Ok(x)) =>
                if &x == token {
                    Ok(())
                } else {
                    Err(UnexpectedToken(x))
                },
        }
    }
}

impl<I : Iterator<TokenResult>> LeftAssociativeBinaryParser<I> {
    fn new(rules: Vec<BinaryRule>) -> LeftAssociativeBinaryParser<I> {
        LeftAssociativeBinaryParser {
            rules: rules,
        }
    }

    fn parse(&self, source: TokenSource<I>, child_parse: |TokenSource<I>| -> ParseResult) -> ParseResult {
        let left = try!(child_parse(source));

        let mut left = match left {
            None => return Ok(None),
            Some(x) => x,
        };

        while source.has_more_tokens() {
            let mut found = false;

            for rule in self.rules.iter() {
                if source.next_token_is(&rule.token) {
                    try!(source.consume(&rule.token));

                    let right = try!(child_parse(source));

                    let right = match right {
                        None => return Err(RightHandSideExpressionMissing),
                        Some(x) => x,
                    };

                    left = (rule.builder)(left, right);

                    found = true;
                    break;
                }
            }

            if !found { break; }
        }

        Ok(Some(left))
    }
}

// std::unique_ptr<XPathExpression>
// parse_children_in_order(std::vector<ParseFn> child_parses, XPathParserTokenSource &source)
// {
//   for (auto child_parse : child_parses) {
//     auto expr = child_parse(source);
//     if (expr) return expr;
//   }

//   return nullptr;
// }

// std::unique_ptr<XPathAxis>
// parse_axis(XPathParserTokenSource &source) {
//   if (source.next_token_is(token::Axis)) {
//     auto token = source.next_token();
//     auto name = token.string();
//     source.consume(token::DoubleColon);

//     if (name == "self") {
//       return make_unique<AxisSelf>();
//     } else if (name == "parent") {
//       return make_unique<AxisParent>();
//     } else if (name == "descendant") {
//       return make_unique<AxisDescendant>();
//     } else if (name == "descendant-or-self") {
//       return make_unique<AxisDescendantOrSelf>();
//     } else if (name == "attribute") {
//       return make_unique<AxisAttribute>();
//     } else {
//       throw InvalidXPathAxisException(name);
//     }
//   }

//   return make_unique<AxisChild>();
// }

// std::unique_ptr<XPathNodeTest>
// parse_node_test(XPathParserTokenSource &source) {
//   if (source.next_token_is(token::NodeTest)) {
//     auto token = source.next_token();
//     auto name = token.string();

//     source.consume(token::LeftParen);
//     source.consume(token::RightParen);

//     if (name == "node") {
//       return make_unique<NodeTestNode>();
//     } else if (name == "text") {
//       return make_unique<NodeTestText>();
//     } else {
//       throw InvalidNodeTestException(name);
//     }
//   }

//   return nullptr;
// }

// std::unique_ptr<XPathNodeTest>
// default_node_test(XPathParserTokenSource &source, std::unique_ptr<XPathAxis> &axis) {
//   if (source.next_token_is(token::String)) {
//     auto token = source.next_token();

//     switch (axis->principal_node_type()) {
//     case PrincipalNodeType::Attribute:
//       return make_unique<NodeTestAttribute>(token.string());
//     case PrincipalNodeType::Element:
//       return make_unique<NodeTestElement>(token.prefixed_name());
//     }
//   }

//   return nullptr;
// }

/// Similar to `consume`, but can be used when the token carries a
/// single value.
macro_rules! consume_value(
    ($source:expr, token::$token:ident) => (
        match $source.next() {
            None => return Err(RanOutOfInput),
            Some(Err(x)) => return Err(TokenizerError(x)),
            Some(Ok(token::$token(x))) => x,
            Some(Ok(x)) => return Err(UnexpectedToken(x)),
        }
    );
)

/// Similar to `next_token_is`, but can be used when the token carries
/// a single value
macro_rules! next_token_is(
    ($source:expr, token::$token:ident) => (
        match $source.peek() {
            Some(&Ok(token::$token(_))) => true,
            _ => false,
        }
    );
)

impl<I : Iterator<TokenResult>> XPathParser {

    fn parse_variable_reference(&self, source: TokenSource<I>) -> ParseResult {
        if source.next_token_is(&token::DollarSign) {
            try!(source.consume(&token::DollarSign));
            let name = consume_value!(source, token::String);
            Ok(Some(box ExpressionVariable { name: name } as SubExpression))
        } else {
            Ok(None)
        }
    }

    fn parse_string_literal(&self, source: TokenSource<I>) -> ParseResult {
        if next_token_is!(source, token::Literal) {
            let value = consume_value!(source, token::Literal);
            Ok(Some(box ExpressionLiteral { value: String(value) } as SubExpression))
        } else {
            Ok(None)
        }
    }

    fn parse_numeric_literal(&self, source: TokenSource<I>) -> ParseResult {
        if next_token_is!(source, token::Number) {
            let value = consume_value!(source, token::Number);
            Ok(Some(box ExpressionLiteral { value: Number(value) } as SubExpression))
        } else {
            Ok(None)
        }
    }

    fn parse_function_call(&self, source: TokenSource<I>) -> ParseResult {
        if next_token_is!(source, token::Function) {
            let name = consume_value!(source, token::Function);

            let mut arguments = Vec::new();

            try!(source.consume(&token::LeftParen));
            while ! source.next_token_is(&token::RightParen) {
                // TODO: this should be the top-level expression
                let arg = try!(self.parse_primary_expression(source));
                match arg {
                    Some(arg) => arguments.push(arg),
                    None => break,
                }
            }
            try!(source.consume(&token::RightParen));

            Ok(Some(box ExpressionFunction{ name: name, arguments: arguments } as SubExpression))
        } else {
            Ok(None)
        }
    }

    fn parse_primary_expression(&self, source: TokenSource<I>) -> ParseResult {
        let mut child_parses = vec![
            |src: TokenSource<I>| self.parse_variable_reference(src),
            |src: TokenSource<I>| self.parse_string_literal(src),
            |src: TokenSource<I>| self.parse_numeric_literal(src),
            |src: TokenSource<I>| self.parse_function_call(src),
        ];

        // TODO: parse_children_in_order
        for child_parse in child_parses.mut_iter() {
            let expr = try!((*child_parse)(source));
            if expr.is_some() {
                return Ok(expr);
            }
        }

        Ok(None)
    }

}

// std::unique_ptr<XPathExpression>
// parse_predicate_expression(XPathParserTokenSource &source)
// {
//   if (source.next_token_is(token::LeftBracket)) {
//     source.consume(token::LeftBracket);

//     // TODO: This should be the top-level expression
//     auto predicate = parse_primary_expression(source);
//     if (! predicate) {
//       throw EmptyPredicateException();
//     }

//     source.consume(token::RightBracket);

//     return predicate;
//   }

//   return nullptr;
// }

// std::unique_ptr<XPathExpression>
// parse_step(XPathParserTokenSource &source)
// {
//   auto axis = parse_axis(source);

//   auto node_test = parse_node_test(source);
//   if (! node_test) {
//     node_test = default_node_test(source, axis);
//   }
//   if (! node_test) {
//     return nullptr;
//   }

//   return make_unique<ExpressionStep>(move(axis), move(node_test));
// }

// std::unique_ptr<XPathExpression>
// parse_predicates(XPathParserTokenSource &source,
//                  std::unique_ptr<XPathExpression> node_selecting_expr)
// {
//   while (auto predicate_expr = parse_predicate_expression(source)) {
//     node_selecting_expr = make_unique<ExpressionPredicate>(move(node_selecting_expr),
//                                                            move(predicate_expr));
//   }

//   return node_selecting_expr;
// }

// std::unique_ptr<XPathExpression>
// parse_relative_location_path_raw(XPathParserTokenSource &source,
//                                  std::unique_ptr<XPathExpression> start_point)
// {
//   std::vector<std::unique_ptr<XPathExpression>> steps;

//   auto step = parse_step(source);
//   if (step) {
//     step = parse_predicates(source, move(step));
//     steps.push_back(move(step));

//     while (source.next_token_is(token::Slash)) {
//       source.consume(token::Slash);

//       auto next = parse_step(source);
//       if (! next) {
//         throw TrailingSlashException();
//       }

//       next = parse_predicates(source, move(next));
//       steps.push_back(move(next));
//     }

//     return make_unique<ExpressionPath>(move(start_point), move(steps));
//   }

//   return nullptr;
// }

// std::unique_ptr<XPathExpression>
// parse_relative_location_path(XPathParserTokenSource &source)
// {
//   auto start_point = make_unique<ExpressionContextNode>();
//   return parse_relative_location_path_raw(source, move(start_point));
// }

// std::unique_ptr<XPathExpression>
// parse_absolute_location_path(XPathParserTokenSource &source)
// {
//   if (source.next_token_is(token::Slash)) {
//     source.consume(token::Slash);

//     auto start_point = make_unique<ExpressionRootNode>();
//     auto expr = parse_relative_location_path_raw(source, move(start_point));
//     if (expr) return expr;

//     return make_unique<ExpressionRootNode>();
//   }

//   return nullptr;
// }

// std::unique_ptr<XPathExpression>
// parse_location_path(XPathParserTokenSource &source)
// {
//   std::vector<ParseFn> child_parses = {
//     parse_relative_location_path,
//     parse_absolute_location_path
//   };

//   return parse_children_in_order(child_parses, source);
// }

// std::unique_ptr<XPathExpression>
// parse_filter_expression(XPathParserTokenSource &source)
// {
//   auto expr = parse_primary_expression(source);
//   if (expr) {
//     return parse_predicates(source, move(expr));
//   }

//   return nullptr;
// }

// std::unique_ptr<XPathExpression>
// parse_path_expression(XPathParserTokenSource &source)
// {
//   auto expr = parse_location_path(source);
//   if (expr) return expr;

//   auto filter = parse_filter_expression(source);
//   if (filter) {
//     if (source.next_token_is(token::Slash)) {
//       source.consume(token::Slash);

//       filter = parse_relative_location_path_raw(source, move(filter));
//       if (! filter) {
//         throw TrailingSlashException();
//       }

//       return filter;
//     }

//     return filter;
//   }

//   return nullptr;
// }

// std::unique_ptr<XPathExpression>
// parse_union_expression(XPathParserTokenSource &source)
// {
//   std::vector<BinaryRule<ExpressionUnion>> rules = {
//     { token::Pipe, ExpressionUnion::Union }
//   };

//   LeftAssociativeBinaryParser<ExpressionUnion> parser(parse_path_expression, rules);
//   return parser.parse(source);
// }

impl<I : Iterator<TokenResult>> XPathParser {

    fn parse_unary_expression(&self, source: TokenSource<I>) -> ParseResult {
        // TODO: reset to parse_union_expression
        let expr = try!(self.parse_primary_expression(source));
        if expr.is_some() {
            return Ok(expr);
        }

        if source.next_token_is(&token::MinusSign) {
            try!(source.consume(&token::MinusSign));

            let expr = try!(self.parse_unary_expression(source));

            match expr {
                Some(expr) => {
                    let expr: SubExpression = box ExpressionNegation { expression: expr };
                    Ok(Some(expr))
                },
                None => Err(RightHandSideExpressionMissing),
            }
        } else {
            Ok(None)
        }
    }

    fn parse_multiplicative_expression(&self, source: TokenSource<I>) -> ParseResult {
        let rules = vec![
            BinaryRule { token: token::Multiply,  builder: ExpressionMath::multiplication },
            BinaryRule { token: token::Divide,    builder: ExpressionMath::division },
            BinaryRule { token: token::Remainder, builder: ExpressionMath::remainder }
        ];

        let parser = LeftAssociativeBinaryParser::new(rules);
        parser.parse(source, |source| self.parse_unary_expression(source))
    }

    fn parse_additive_expression(&self, source: TokenSource<I>) -> ParseResult {
        let rules = vec![
            BinaryRule { token: token::PlusSign,  builder: ExpressionMath::addition },
            BinaryRule { token: token::MinusSign, builder: ExpressionMath::subtraction}
        ];

        let parser = LeftAssociativeBinaryParser::new(rules);
        parser.parse(source, |source| self.parse_multiplicative_expression(source))
    }

    fn parse_relational_expression(&self, source: TokenSource<I>) -> ParseResult {
        let rules = vec![
            BinaryRule { token: token::LessThan,           builder: ExpressionRelational::less_than },
            BinaryRule { token: token::LessThanOrEqual,    builder: ExpressionRelational::less_than_or_equal },
            BinaryRule { token: token::GreaterThan,        builder: ExpressionRelational::greater_than },
            BinaryRule { token: token::GreaterThanOrEqual, builder: ExpressionRelational::greater_than_or_equal },
        ];

        let parser = LeftAssociativeBinaryParser::new(rules);
        parser.parse(source, |source| self.parse_additive_expression(source))
    }

    fn parse_equality_expression(&self, source: TokenSource<I>) -> ParseResult {
        let rules = vec![
            BinaryRule { token: token::Equal,    builder: ExpressionEqual::new },
            BinaryRule { token: token::NotEqual, builder: ExpressionNotEqual::new },
        ];

        let parser = LeftAssociativeBinaryParser::new(rules);
        parser.parse(source, |source| self.parse_relational_expression(source))
    }

    fn parse_and_expression(&self, source: TokenSource<I>) -> ParseResult {
        let rules = vec![
            BinaryRule { token: token::And, builder: ExpressionAnd::new }
        ];

        let parser = LeftAssociativeBinaryParser::new(rules);
        parser.parse(source, |source| self.parse_equality_expression(source))
    }

    fn parse_or_expression(&self, source: TokenSource<I>) -> ParseResult
    {
        let rules = vec![
            BinaryRule { token: token::Or, builder: ExpressionOr::new }
        ];

        let parser = LeftAssociativeBinaryParser::new(rules);
        parser.parse(source, |source| self.parse_and_expression(source))
    }

    pub fn parse(&self, source: I) -> ParseResult {
        let mut source = source.peekable();

        let expr = try!(self.parse_or_expression(&mut source));

        if source.has_more_tokens() {
            return Err(ExtraUnparsedTokens);
        }

        Ok(expr)
    }
}
