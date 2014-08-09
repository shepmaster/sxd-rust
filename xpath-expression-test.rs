extern crate xpath;

use xpath::XPathValue;
use xpath::{Boolean, Number, String, Nodes};
use xpath::Node;
use xpath::expression::XPathExpression;
use xpath::expression::{ExpressionAnd, ExpressionEqual, ExpressionNotEqual};
use xpath::XPathEvaluationContext;

struct StubExpression {
    value: XPathValue,
}

impl XPathExpression for StubExpression {
    fn evaluate(&self, _: &XPathEvaluationContext) -> XPathValue {
        self.value.clone()
    }
}

struct FailExpression;

impl XPathExpression for FailExpression {
    fn evaluate(&self, _: &XPathEvaluationContext) -> XPathValue {
        fail!("Should never be called");
    }
}

#[test]
fn expression_and_returns_logical_and() {
    let left  = box StubExpression{value: Boolean(true)};
    let right = box StubExpression{value: Boolean(true)};

    let node = Node::new();
    let context = XPathEvaluationContext {node: &node};
    let expr = ExpressionAnd{left: left, right: right};

    let res = expr.evaluate(&context);

    assert_eq!(res, Boolean(true));
}

#[test]
fn expression_and_short_circuits_when_left_argument_is_false() {
    let left  = box StubExpression{value: Boolean(false)};
    let right = box FailExpression;

    let node = Node::new();
    let context = XPathEvaluationContext {node: &node};
    let expr = ExpressionAnd{left: left, right: right};

    expr.evaluate(&context);
    // assert_not_fail
}

#[test]
fn expression_equal_compares_as_boolean_if_one_argument_is_a_boolean() {
    let actual_bool = box StubExpression{value: Boolean(false)};
    let truthy_str = box StubExpression{value: String("hello".to_string())};

    let node = Node::new();
    let context = XPathEvaluationContext {node: &node};
    let expr = ExpressionEqual{left: actual_bool, right: truthy_str};

    let res = expr.evaluate(&context);

    assert_eq!(res, Boolean(false));
}

#[test]
fn expression_equal_compares_as_number_if_one_argument_is_a_number() {
    let actual_number = box StubExpression{value: Number(-42.0)};
    let number_str = box StubExpression{value: String("-42.0".to_string())};

    let node = Node::new();
    let context = XPathEvaluationContext {node: &node};
    let expr = ExpressionEqual{left: number_str, right: actual_number};

    let res = expr.evaluate(&context);
    assert_eq!(res, Boolean(true));
}

#[test]
fn expression_equal_compares_as_string_otherwise() {
    let a_str = box StubExpression{value: String("hello".to_string())};
    let b_str = box StubExpression{value: String("World".to_string())};

    let node = Node::new();
    let context = XPathEvaluationContext {node: &node};
    let expr = ExpressionEqual{left: a_str, right: b_str};

    let res = expr.evaluate(&context);
    assert_eq!(res, Boolean(false));
}

#[test]
fn expression_not_equal_negates_equality() {
    let a_str = box StubExpression{value: Boolean(true)};
    let b_str = box StubExpression{value: Boolean(false)};

    let node = Node::new();
    let context = XPathEvaluationContext {node: &node};
    let expr = ExpressionNotEqual::new(a_str, b_str);

    let res = expr.evaluate(&context);
    assert_eq!(res, Boolean(true));
}
