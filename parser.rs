use std::iter::Peekable;

use super::Number;
use super::token;
use super::token::XPathToken;
use super::tokenizer::TokenResult;
use super::expression::XPathExpression;
use super::expression::{ExpressionEqual,
                        ExpressionLiteral,
                        ExpressionNotEqual};

pub struct XPathParser;

impl XPathParser {
    pub fn new() -> XPathParser {
        XPathParser
    }
}

pub type SubExpression = Box<XPathExpression + 'static>;

#[deriving(Show,PartialEq,Clone)]
pub enum ParseErr {
    RightHandSideExpressionMissing,
    ExtraUnparsedTokens,
}

pub type ParseResult = Result<Option<SubExpression>, ParseErr>;

type ParseFn<I> = fn(XPathParserTokenSource<I>) -> ParseResult;
type BinaryExpressionBuilder = fn(SubExpression, SubExpression) -> SubExpression;

struct BinaryRule {
    token_type: XPathToken,
    builder: BinaryExpressionBuilder,
}

struct LeftAssociativeBinaryParser<I> {
    child_parse: ParseFn<I>,
    rules: Vec<BinaryRule>,
}

type XPathParserTokenSource<'a, I> = &'a mut Peekable<TokenResult, I>;

trait XCompat {
    fn has_more_tokens(&mut self) -> bool;
    fn next_token_is(&mut self, token: &XPathToken) -> bool;
    fn consume(&mut self, token: &XPathToken);
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

    fn consume(&mut self, token: &XPathToken) {
        if ! self.next_token_is(token) {
            fail!("Expected token was not found!");
        }
        self.next();
    }
}

impl<I : Iterator<TokenResult>> LeftAssociativeBinaryParser<I> {
    fn new(child_parse: ParseFn<I>, rules: Vec<BinaryRule>) -> LeftAssociativeBinaryParser<I> {
        LeftAssociativeBinaryParser {
            child_parse: child_parse,
            rules: rules,
        }
    }

    fn parse(&self, source: &mut Peekable<TokenResult, I>) -> ParseResult {
        let left = try!((self.child_parse)(source));

        let mut left = match left {
            None => return Ok(None),
            Some(x) => x,
        };

        while source.has_more_tokens() {
            let mut found = false;

            for rule in self.rules.iter() {
                if source.next_token_is(&rule.token_type) {
                    source.consume(&rule.token_type);

                    let right = try!((self.child_parse)(source));

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

// std::unique_ptr<XPathExpression>
// parse_variable_reference(XPathParserTokenSource &source)
// {
//   if (source.next_token_is(token::DollarSign)) {
//     source.consume(token::DollarSign);
//     auto token = source.consume(token::String);

//     return make_unique<ExpressionVariable>(token.string());
//   }

//   return nullptr;
// }

// std::unique_ptr<XPathExpression>
// parse_string_literal(XPathParserTokenSource &source) {
//   if (source.next_token_is(token::Literal)) {
//     auto token = source.next_token();
//     return make_unique<ExpressionLiteral>(token.string());
//   }

//   return nullptr;
// }

fn parse_numeric_literal<I : Iterator<TokenResult>>(source: &mut Peekable<TokenResult, I>) -> ParseResult {
    match source.peek() {
        Some(&Ok(token::Number(_))) => {}
        _ => return Ok(None),
    };

    // This is ugly
    let token = source.next().unwrap().unwrap();
    match token {
        token::Number(v) => {
            let expr: Box<XPathExpression> = box ExpressionLiteral { value: Number(v) };
            Ok(Some(expr))
        },
        _ => fail!("A number wasn't a number!"),
    }
}

// std::unique_ptr<XPathExpression>
// parse_primary_expression(XPathParserTokenSource &source);

// std::unique_ptr<XPathExpression>
// parse_function_call(XPathParserTokenSource &source)
// {
//   if (source.next_token_is(token::Function)) {
//     auto token = source.next_token();

//     std::vector<std::shared_ptr<XPathExpression>> arguments;

//     source.consume(token::LeftParen);
//     while (! source.next_token_is(token::RightParen)) {
//       // TODO: this should be the top-level expression
//       auto arg = parse_primary_expression(source);
//       if (! arg) break;
//       arguments.push_back(move(arg));
//     }
//     source.consume(token::RightParen);

//     return make_unique<ExpressionFunction>(token.string(), std::move(arguments));
//   }

//   return nullptr;
// }

// std::unique_ptr<XPathExpression>
// parse_primary_expression(XPathParserTokenSource &source) {
//   std::vector<ParseFn> child_parses = {
//     parse_variable_reference,
//     parse_string_literal,
//     parse_numeric_literal,
//     parse_function_call
//   };

//   return parse_children_in_order(child_parses, source);
// }

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

// std::unique_ptr<XPathExpression>
// parse_unary_expression(XPathParserTokenSource &source)
// {
//   auto expr = parse_union_expression(source);
//   if (expr) return expr;

//   if (source.next_token_is(token::MinusSign)) {
//     source.consume(token::MinusSign);

//     expr = parse_unary_expression(source);
//     if (! expr) {
//       throw RightHandSideExpressionMissingException();
//     }

//     return make_unique<ExpressionNegation>(move(expr));
//   }

//   return nullptr;
// }

// std::unique_ptr<XPathExpression>
// parse_multiplicative_expression(XPathParserTokenSource &source)
// {
//   std::vector<BinaryRule<ExpressionMath>> rules = {
//     { token::Multiply,  ExpressionMath::Multiplication },
//     { token::Divide,    ExpressionMath::Division },
//     { token::Remainder, ExpressionMath::Remainder }
//   };

//   LeftAssociativeBinaryParser<ExpressionMath> parser(parse_unary_expression, rules);
//   return parser.parse(source);
// }

// std::unique_ptr<XPathExpression>
// parse_additive_expression(XPathParserTokenSource &source)
// {
//   std::vector<BinaryRule<ExpressionMath>> rules = {
//     { token::PlusSign,  ExpressionMath::Addition },
//     { token::MinusSign, ExpressionMath::Subtraction}
//   };

//   LeftAssociativeBinaryParser<ExpressionMath> parser(parse_multiplicative_expression, rules);
//   return parser.parse(source);
// }

// std::unique_ptr<XPathExpression>
// parse_relational_expression(XPathParserTokenSource &source)
// {
//   std::vector<BinaryRule<ExpressionRelational>> rules = {
//     { token::LessThan,           ExpressionRelational::LessThan },
//     { token::LessThanOrEqual,    ExpressionRelational::LessThanOrEqual },
//     { token::GreaterThan,        ExpressionRelational::GreaterThan },
//     { token::GreaterThanOrEqual, ExpressionRelational::GreaterThanOrEqual },
//   };

//   LeftAssociativeBinaryParser<ExpressionRelational> parser(parse_additive_expression, rules);
//   return parser.parse(source);
// }

fn parse_equality_expression<I : Iterator<TokenResult>>(source: &mut Peekable<TokenResult, I>) -> ParseResult {
    let rules = vec![
        BinaryRule { token_type: token::Equal,    builder: ExpressionEqual::new },
        BinaryRule { token_type: token::NotEqual, builder: ExpressionNotEqual::new },
    ];

    // TODO reset to parse_relational_expression
    let parser = LeftAssociativeBinaryParser::new(parse_numeric_literal, rules);
    return parser.parse(source);
}

// std::unique_ptr<XPathExpression>
// parse_and_expression(XPathParserTokenSource &source)
// {
//   std::vector<BinaryRule<ExpressionAnd>> rules = {
//     { token::And, ExpressionAnd::And }
//   };

//   LeftAssociativeBinaryParser<ExpressionAnd> parser(parse_equality_expression, rules);
//   return parser.parse(source);
// }

// std::unique_ptr<XPathExpression>
// parse_or_expression(XPathParserTokenSource &source)
// {
//   std::vector<BinaryRule<ExpressionOr>> rules = {
//     { token::Or, ExpressionOr::Or }
//   };

//   LeftAssociativeBinaryParser<ExpressionOr> parser(parse_and_expression, rules);
//   return parser.parse(source);
// }

impl<I : Iterator<TokenResult>> XPathParser {
    pub fn parse(&self, source: I) -> ParseResult {
        let mut source = source.peekable();

        // TODO: reset to parse_or_expression
        let expr = try!(parse_equality_expression(&mut source));

        if source.has_more_tokens() {
            return Err(ExtraUnparsedTokens);
        }

        Ok(expr)
    }
}
