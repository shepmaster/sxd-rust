#![feature(phase)]
#![feature(macro_rules)]

#[phase(plugin, link)]
extern crate document;
extern crate xpath;

use std::collections::hashmap::HashMap;

use document::{Document,Element,ToAny};

use xpath::{Boolean,Number,String,Nodes};
use xpath::token;
use xpath::tokenizer::TokenResult;
use xpath::{Functions,Variables};
use xpath::{XPathValue,XPathEvaluationContext};
use xpath::expression::XPathExpression;
use xpath::parser::XPathParser;
use xpath::parser::{
    RightHandSideExpressionMissing,
    UnexpectedToken,
};

macro_rules! tokens(
    ($($e:expr),*) => ({
        // leading _ to allow empty construction without a warning.
        let mut _temp: Vec<TokenResult> = ::std::vec::Vec::new();
        $(_temp.push(Ok($e));)*
        _temp
    });
    ($($e:expr),+,) => (tokens!($($e),+))
)

trait ApproxEq {
    fn is_approx_eq(&self, other: &Self) -> bool;
}

impl ApproxEq for f64 {
    fn is_approx_eq(&self, other: &f64) -> bool {
        (*self - *other).abs() < 1.0e-6
    }
}

impl ApproxEq for XPathValue {
    fn is_approx_eq(&self, other: &XPathValue) -> bool {
        match (self, other) {
            (&Number(ref x), &Number(ref y)) => x.is_approx_eq(y),
            _ => fail!("It's nonsensical to compare these quantities"),
        }
    }
}

macro_rules! assert_approx_eq(
    ($a:expr, $b:expr) => ({
        let (a, b) = (&$a, &$b);
        assert!(a.is_approx_eq(b),
                "{} is not approximately equal to {}", *a, *b);
    })
)

// class XPathParserTest : public ::testing::Test {
// protected:
//   TokenProvider tokens;

//   NullNamespaceResolver null_namespaces;

//   void SetUp() {
//     XPathCoreFunctionLibrary::register_functions(functions);
//   }

//   Attribute *add_attribute(Element *element, std::string name, std::string value) {
//     return element->set_attribute(name, value);
//   }

//   TextNode *add_text_node(Element *parent, std::string value) {
//     auto tn = doc.new_text_node(value);
//     parent->append_child(tn);
//     return tn;
//   }
// };

struct Setup {
    doc: Document,
    top_node: Element,
    functions: Functions,
    variables: Variables,
    parser: XPathParser,
}

impl Setup {
    fn new() -> Setup {
        let d = Document::new();
        let e = d.new_element("the-top-node".to_string());
        d.root().append_child(e.clone());

        let mut functions = HashMap::new();
        xpath::function::register_core_functions(& mut functions);

        Setup {
            doc: d,
            top_node: e,
            functions: functions,
            variables: HashMap::new(),
            parser: XPathParser::new(),
        }
    }

    fn add_child(&self, parent: &Element, name: &str) -> Element {
        let n = self.doc.new_element(name.to_string());
        parent.append_child(n.clone());
        n
  }

    fn evaluate(&self, expr: &XPathExpression) -> XPathValue {
        let mut context = XPathEvaluationContext::new(self.top_node.to_any(), &self.functions, &self.variables);
        context.next(self.top_node.to_any());
        expr.evaluate(&context)
    }
}

// #[test]
// fn parses_string_as_child)
// {
//   tokens.add(token::String("hello"));

//   auto expr = parser->parse();

//   auto hello = add_child(top_node, "hello");

//   ASSERT_THAT(evaluate_on(expr, top_node).nodeset(), ElementsAre(hello));
// }

// #[test]
// fn parses_two_strings_as_grandchild)
// {
//   tokens.add({
//       token::String("hello"),
//       token::Slash,
//       token::String("world")
//   });

//   auto expr = parser->parse();

//   auto hello = add_child(top_node, "hello");
//   auto world = add_child(hello, "world");

//   ASSERT_THAT(evaluate_on(expr, top_node).nodeset(), ElementsAre(world));
// }

// #[test]
// fn parses_self_axis)
// {
//   tokens.add({
//       token::Axis, "self",
//       token::DoubleColon,
//       token::String("top-node")
//   });

//   auto expr = parser->parse();

//   ASSERT_THAT(evaluate_on(expr, top_node).nodeset(), ElementsAre(top_node));
// }

// #[test]
// fn parses_parent_axis)
// {
//   tokens.add({
//       token::Axis, "parent",
//       token::DoubleColon,
//       token::String("top-node")
//   });

//   auto expr = parser->parse();

//   auto hello = add_child(top_node, "hello");
//   ASSERT_THAT(evaluate_on(expr, hello).nodeset(), ElementsAre(top_node));
// }

// #[test]
// fn parses_descendant_axis)
// {
//   tokens.add({
//       token::Axis, "descendant",
//       token::DoubleColon,
//       token::String("two")
//   });

//   auto expr = parser->parse();

//   auto one = add_child(top_node, "one");
//   auto two = add_child(one, "two");

//   ASSERT_THAT(evaluate_on(expr, top_node).nodeset(), ElementsAre(two));
// }

// #[test]
// fn parses_descendant_or_self_axis)
// {
//   tokens.add({
//       token::Axis, "descendant-or-self",
//       token::DoubleColon,
//       token::String("*")
//   });

//   auto expr = parser->parse();

//   auto one = add_child(top_node, "one");
//   auto two = add_child(one, "two");

//   ASSERT_THAT(evaluate_on(expr, one).nodeset(), ElementsAre(one, two));
// }

// #[test]
// fn parses_attribute_axis)
// {
//   tokens.add({
//       token::Axis, "attribute",
//       token::DoubleColon,
//       token::String("*")
//   });

//   auto expr = parser->parse();

//   auto one = add_child(top_node, "one");
//   auto attr = add_attribute(one, "hello", "world");

//   ASSERT_THAT(evaluate_on(expr, one).nodeset(), ElementsAre(attr));
// }

// #[test]
// fn parses_child_with_same_name_as_an_axis)
// {
//   tokens.add(token::String("self"));

//   auto expr = parser->parse();

//   auto self = add_child(top_node, "self");
//   ASSERT_THAT(evaluate_on(expr, top_node).nodeset(), ElementsAre(self));
// }

// #[test]
// fn parses_node_node_test)
// {
//   tokens.add({
//       token::NodeTest, "node",
//       token::LeftParen,
//       token::RightParen
//   });

//   auto expr = parser->parse();

//   auto one = add_child(top_node, "one");
//   auto two = add_child(one, "two");

//   ASSERT_THAT(evaluate_on(expr, one).nodeset(), ElementsAre(two));
// }

// #[test]
// fn parses_text_node_test)
// {
//   tokens.add({
//       token::NodeTest, "text",
//       token::LeftParen,
//       token::RightParen
//   });

//   auto expr = parser->parse();

//   auto one = add_child(top_node, "one");
//   auto text = add_text_node(one, "text");

//   ASSERT_THAT(evaluate_on(expr, one).nodeset(), ElementsAre(text));
// }

// #[test]
// fn parses_axis_and_node_test)
// {
//   tokens.add({
//       token::Axis, "self",
//       token::DoubleColon,
//       token::NodeTest, "text",
//       token::LeftParen,
//       token::RightParen
//   });

//   auto expr = parser->parse();

//   auto one = add_child(top_node, "one");
//   auto text = add_text_node(one, "text");

//   ASSERT_THAT(evaluate_on(expr, text).nodeset(), ElementsAre(text));
// }

// #[test]
// fn numeric_predicate_selects_indexed_node)
// {
//   tokens.add({
//       token::String("*"),
//       token::LeftBracket,
//       token::Number(2),
//       token::RightBracket
//   });

//   auto expr = parser->parse();

//   add_child(top_node, "first");
//   auto second = add_child(top_node, "second");

//   ASSERT_THAT(evaluate_on(expr, top_node).nodeset(), ElementsAre(second));
// }

#[test]
fn string_literal() {
    let setup = Setup::new();
    let tokens = tokens![token::Literal("string".to_string())];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_eq!(String("string".to_string()), setup.evaluate(expr));
}

// #[test]
// fn true_function_predicate_selects_all_nodes)
// {
//   tokens.add({
//       token::String("*"),
//       token::LeftBracket,
//       token::Function, "true",
//       token::LeftParen,
//       token::RightParen,
//       token::RightBracket
//   });

//   auto expr = parser->parse();

//   auto first = add_child(top_node, "first");
//   auto second = add_child(top_node, "second");

//   ASSERT_THAT(evaluate_on(expr, top_node).nodeset(), ElementsAre(first, second));
// }

// #[test]
// fn false_function_predicate_selects_no_nodes)
// {
//   tokens.add({
//       token::String("*"),
//       token::LeftBracket,
//       token::Function, "false",
//       token::LeftParen,
//       token::RightParen,
//       token::RightBracket
//   });

//   auto expr = parser->parse();

//   add_child(top_node, "first");
//   add_child(top_node, "second");

//   ASSERT_THAT(evaluate_on(expr, top_node).nodeset(), ElementsAre());
// }

// #[test]
// fn multiple_predicates)
// {
//   tokens.add({
//       token::String("*"),
//       token::LeftBracket,
//       token::Number(2.0),
//       token::RightBracket,
//       token::LeftBracket,
//       token::Number(1.0),
//       token::RightBracket
//   });

//   auto expr = parser->parse();

//   add_child(top_node, "first");
//   auto second = add_child(top_node, "second");

//   ASSERT_THAT(evaluate_on(expr, top_node).nodeset(), ElementsAre(second));
// }

#[test]
fn functions_accept_arguments() {
    let setup = Setup::new();
    let tokens = tokens![
      token::Function("not".to_string()),
      token::LeftParen,
      token::Function("true".to_string()),
      token::LeftParen,
      token::RightParen,
      token::RightParen,
  ];

  let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

  assert_eq!(Boolean(false), setup.evaluate(expr));
}

#[test]
fn numeric_literal() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(3.2),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_approx_eq!(Number(3.2), setup.evaluate(expr));
}

#[test]
fn addition_of_two_numbers() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(1.1),
        token::PlusSign,
        token::Number(2.2)
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_approx_eq!(Number(3.3), setup.evaluate(expr));
}

#[test]
fn addition_of_multiple_numbers() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(1.1),
        token::PlusSign,
        token::Number(2.2),
        token::PlusSign,
        token::Number(3.3)
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_approx_eq!(Number(6.6), setup.evaluate(expr));
}

#[test]
fn subtraction_of_two_numbers() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(1.1),
        token::MinusSign,
        token::Number(2.2),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_approx_eq!(Number(-1.1), setup.evaluate(expr));
}

#[test]
fn additive_expression_is_left_associative() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(1.1),
        token::MinusSign,
        token::Number(2.2),
        token::MinusSign,
        token::Number(3.3),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_approx_eq!(Number(-4.4), setup.evaluate(expr));
}

#[test]
fn multiplication_of_two_numbers() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(1.1),
        token::Multiply,
        token::Number(2.2),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_approx_eq!(Number(2.42), setup.evaluate(expr));
}

#[test]
fn division_of_two_numbers() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(7.1),
        token::Divide,
        token::Number(0.1),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_approx_eq!(Number(71.0), setup.evaluate(expr));
}

#[test]
fn remainder_of_two_numbers() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(7.1),
        token::Remainder,
        token::Number(3.0),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_approx_eq!(Number(1.1), setup.evaluate(expr));
}

#[test]
fn unary_negation() {
    let setup = Setup::new();
    let tokens = tokens![
        token::MinusSign,
        token::Number(7.2),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_approx_eq!(Number(-7.2), setup.evaluate(expr));
}

#[test]
fn repeated_unary_negation() {
    let setup = Setup::new();
    let tokens = tokens![
        token::MinusSign,
        token::MinusSign,
        token::MinusSign,
        token::Number(7.2),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_approx_eq!(Number(-7.2), setup.evaluate(expr));
}

#[test]
fn top_level_function_call() {
    let setup = Setup::new();
    let tokens = tokens![
      token::Function("true".to_string()),
      token::LeftParen,
      token::RightParen,
  ];

  let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

  assert_eq!(Boolean(true), setup.evaluate(expr));
}

#[test]
fn or_expression() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Function("true".to_string()),
        token::LeftParen,
        token::RightParen,
        token::Or,
        token::Function("false".to_string()),
        token::LeftParen,
        token::RightParen,
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_eq!(Boolean(true), setup.evaluate(expr));
}

#[test]
fn and_expression() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(1.2),
        token::And,
        token::Number(0.0),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_eq!(Boolean(false), setup.evaluate(expr));
}

#[test]
fn equality_expression() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(1.2),
        token::Equal,
        token::Number(1.1),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_eq!(Boolean(false), setup.evaluate(expr));
}

#[test]
fn inequality_expression() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(1.2),
        token::NotEqual,
        token::Number(1.2),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_eq!(Boolean(false), setup.evaluate(expr));
}

#[test]
fn less_than_expression() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(1.2),
        token::LessThan,
        token::Number(1.2),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_eq!(Boolean(false), setup.evaluate(expr));
}

#[test]
fn less_than_or_equal_expression() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(1.2),
        token::LessThanOrEqual,
        token::Number(1.2),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_eq!(Boolean(true), setup.evaluate(expr));
}

#[test]
fn greater_than_expression() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(1.2),
        token::GreaterThan,
        token::Number(1.2),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_eq!(Boolean(false), setup.evaluate(expr));
}

#[test]
fn greater_than_or_equal_expression() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Number(1.2),
        token::GreaterThanOrEqual,
        token::Number(1.2),
    ];

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_eq!(Boolean(true), setup.evaluate(expr));
}

#[test]
fn variable_reference() {
    let mut setup = Setup::new();
    let tokens = tokens![
      token::DollarSign,
      token::String("variable-name".to_string()),
  ];

  setup.variables.insert("variable-name".to_string(), Number(12.3));
  let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

  assert_approx_eq!(Number(12.3), setup.evaluate(expr));
}

// #[test]
// fn filter_expression)
// {
//   tokens.add({
//       token::DollarSign,
//       token::String("variable"),
//       token::LeftBracket,
//       token::Number(0),
//       token::RightBracket,
//   });

//   Nodeset value;
//   value.add(add_child(top_node, "first-node"));
//   value.add(add_child(top_node, "second-node"));
//   variables.set("variable", value);

//   auto expr = parser->parse();

//   ASSERT_THAT(evaluate(expr).nodeset(), ElementsAre());
// }

// #[test]
// fn filter_expression_and_relative_path)
// {
//   tokens.add({
//       token::DollarSign,
//       token::String("variable"),
//       token::Slash,
//       token::String("child"),
//   });

//   auto parent = add_child(top_node, "parent");
//   auto child = add_child(parent, "child");

//   Nodeset variable_value;
//   variable_value.add(parent);
//   variables.set("variable", variable_value);

//   auto expr = parser->parse();

//   ASSERT_THAT(evaluate(expr).nodeset(), ElementsAre(child));
// }

#[test]
fn union_expression()
{
    let mut setup = Setup::new();
    let tokens = tokens![
        token::DollarSign,
        token::String("variable1".to_string()),
        token::Pipe,
        token::DollarSign,
        token::String("variable2".to_string()),
    ];

    let node1 = setup.add_child(&setup.top_node, "first-node");
    let value1 = nodeset![node1.clone()];
    setup.variables.insert("variable1".to_string(), Nodes(value1));

    let node2 = setup.add_child(&setup.top_node, "second-node");
    let value2 = nodeset![node2.clone()];
    setup.variables.insert("variable2".to_string(), Nodes(value2));

    let expr = setup.parser.parse(tokens.move_iter()).unwrap().unwrap();

    assert_eq!(Nodes(nodeset![node1, node2]), setup.evaluate(expr));
}

// #[test]
// fn absolute_path_expression)
// {
//   tokens.add({
//       token::Slash,
//   });

//   auto node1 = add_child(top_node, "first-node");
//   auto node2 = add_child(node1, "second-node");

//   auto expr = parser->parse();

//   ASSERT_THAT(evaluate_on(expr, node2).nodeset(), ElementsAre(doc.root()));
// }

// #[test]
// fn absolute_path_with_child_expression)
// {
//   tokens.add({
//       token::Slash,
//       token::String("*"),
//   });

//   auto node1 = add_child(top_node, "first-node");
//   auto node2 = add_child(node1, "second-node");

//   auto expr = parser->parse();

//   ASSERT_THAT(evaluate_on(expr, node2).nodeset(), ElementsAre(top_node));
// }

// #[test]
// fn unknown_axis_is_reported_as_an_error)
// {
//   tokens.add({
//       token::Axis, "bad-axis",
//       token::DoubleColon,
//       token::String("*")
//   });

//   ASSERT_THROW(parser->parse(), InvalidXPathAxisException);
// }

// #[test]
// fn unknown_node_test_is_reported_as_an_error)
// {
//   tokens.add({
//       token::NodeTest, "bad-node-test",
//       token::LeftParen,
//       token::RightParen
//   });

//   ASSERT_THROW(parser->parse(), InvalidNodeTestException);
// }

#[test]
fn unexpected_token_is_reported_as_an_error() {
    let setup = Setup::new();
    let tokens = tokens![
        token::Function("does-not-matter".to_string()),
        token::RightParen
    ];

    let res = setup.parser.parse(tokens.move_iter());

    assert_eq!(Some(UnexpectedToken(token::RightParen)), res.err());
}

// #[test]
// fn binary_operator_without_right_hand_side_is_reported_as_an_error)
// {
//   tokens.add({
//       token::Literal, "left",
//       token::And
//   });

//   ASSERT_THROW(parser->parse(), RightHandSideExpressionMissingException);
// }

#[test]
fn unary_operator_without_right_hand_side_is_reported_as_an_error() {
    let setup = Setup::new();
    let tokens = tokens![
        token::MinusSign,
    ];

    let res = setup.parser.parse(tokens.move_iter());

    assert_eq!(Some(RightHandSideExpressionMissing), res.err());
}

// #[test]
// fn empty_predicate_is_reported_as_an_error)
// {
//   tokens.add({
//       token::String("*"),
//       token::LeftBracket,
//       token::RightBracket,
//   });

//   ASSERT_THROW(parser->parse(), EmptyPredicateException);
// }

// #[test]
// fn relative_path_with_trailing_slash_is_reported_as_an_error)
// {
//   tokens.add({
//       token::String("*"),
//       token::Slash,
//   });

//   ASSERT_THROW(parser->parse(), TrailingSlashException);
// }

// #[test]
// fn filter_expression_with_trailing_slash_is_reported_as_an_error)
// {
//   tokens.add({
//       token::DollarSign,
//       token::String("variable"),
//       token::Slash,
//   });

//   ASSERT_THROW(parser->parse(), TrailingSlashException);
// }

// int main(int argc, char **argv) {
//   ::testing::InitGoogleTest(&argc, argv);
//   return RUN_ALL_TESTS();
// }
