extern crate xpath;

use xpath::XPathValue;
use xpath::Boolean;
use xpath::Node;
use xpath::expression::XPathExpression;
use xpath::expression::ExpressionAnd;
use xpath::XPathEvaluationContext;

struct StubExpression {
    value: XPathValue,
}

impl XPathExpression for StubExpression {
    fn evaluate(&self, _: &XPathEvaluationContext) -> XPathValue {
        self.value
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
