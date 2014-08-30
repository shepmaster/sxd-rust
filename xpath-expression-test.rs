extern crate document;
extern crate xpath;

use std::collections::HashMap;

use document::{Document,Element,Nodeset};

use xpath::XPathValue;
use xpath::{Boolean, Number, String, Nodes};
use xpath::expression::XPathExpression;
use xpath::expression::{ExpressionAnd,
                        ExpressionEqual,
                        ExpressionNotEqual,
                        ExpressionFunction,
                        ExpressionLiteral,
                        ExpressionMath,
                        ExpressionPredicate,
                        ExpressionRelational,
                        ExpressionRootNode};
use xpath::XPathFunction;
use xpath::XPathEvaluationContext;

struct FailExpression;

impl<'n> XPathExpression<'n> for FailExpression {
    fn evaluate(&self, _: &XPathEvaluationContext<'n>) -> XPathValue<'n> {
        fail!("Should never be called");
    }
}

struct Setup<'a> {
    doc: Document,
    node: Element,
    funs: HashMap<String, Box<XPathFunction<'a>>>,
}

impl<'a> Setup<'a> {
    fn new() -> Setup<'a> {
        let d = Document::new();
        let n = d.new_element("test".to_string());
        Setup {
            doc: d,
            node: n,
            funs: HashMap::new(),
        }
    }

    fn context(& 'a self) -> XPathEvaluationContext<'a> {
        XPathEvaluationContext::new(self.node.clone(), &self.funs)
    }
}

#[test]
fn expression_and_returns_logical_and() {
    let setup = Setup::new();

    let left  = box ExpressionLiteral{value: Boolean(true)};
    let right = box ExpressionLiteral{value: Boolean(true)};

    let expr = ExpressionAnd{left: left, right: right};

    let context = setup.context();
    let res = expr.evaluate(&context);

    assert_eq!(res, Boolean(true));
}

#[test]
fn expression_and_short_circuits_when_left_argument_is_false() {
    let setup = Setup::new();

    let left  = box ExpressionLiteral{value: Boolean(false)};
    let right = box FailExpression;

    let expr = ExpressionAnd{left: left, right: right};

    let context = setup.context();
    expr.evaluate(&context);
    // assert_not_fail
}

#[test]
fn expression_equal_compares_as_boolean_if_one_argument_is_a_boolean() {
    let setup = Setup::new();

    let actual_bool = box ExpressionLiteral{value: Boolean(false)};
    let truthy_str = box ExpressionLiteral{value: String("hello".to_string())};

    let expr = ExpressionEqual{left: actual_bool, right: truthy_str};

    let context = setup.context();
    let res = expr.evaluate(&context);

    assert_eq!(res, Boolean(false));
}

#[test]
fn expression_equal_compares_as_number_if_one_argument_is_a_number() {
    let setup = Setup::new();

    let actual_number = box ExpressionLiteral{value: Number(-42.0)};
    let number_str = box ExpressionLiteral{value: String("-42.0".to_string())};

    let expr = ExpressionEqual{left: number_str, right: actual_number};

    let context = setup.context();
    let res = expr.evaluate(&context);

    assert_eq!(res, Boolean(true));
}

#[test]
fn expression_equal_compares_as_string_otherwise() {
    let setup = Setup::new();

    let a_str = box ExpressionLiteral{value: String("hello".to_string())};
    let b_str = box ExpressionLiteral{value: String("World".to_string())};

    let expr = ExpressionEqual{left: a_str, right: b_str};

    let context = setup.context();
    let res = expr.evaluate(&context);

    assert_eq!(res, Boolean(false));
}

#[test]
fn expression_not_equal_negates_equality() {
    let setup = Setup::new();

    let a_str = box ExpressionLiteral{value: Boolean(true)};
    let b_str = box ExpressionLiteral{value: Boolean(false)};

    let expr = ExpressionNotEqual::new(a_str, b_str);

    let context = setup.context();
    let res = expr.evaluate(&context);

    assert_eq!(res, Boolean(true));
}

struct StubFunction<'n> {
    value: XPathValue<'n>,
}

impl<'n> XPathFunction<'n> for StubFunction<'n> {
    fn evaluate(&self,
                _: &XPathEvaluationContext,
                _: Vec<XPathValue>) -> XPathValue<'n>
    {
        self.value.clone()
    }
}

#[test]
fn expression_function_evaluates_input_arguments() {
    let mut setup = Setup::new();

    let arg_expr: Box<XPathExpression> = box ExpressionLiteral{value: Boolean(true)};
    let fun = box StubFunction{value: String("the function ran".to_string())};
    setup.funs.insert("test-fn".to_string(), fun);

    let expr = ExpressionFunction{name: "test-fn".to_string(), arguments: vec!(arg_expr)};

    let context = setup.context();
    let res = expr.evaluate(&context);

    assert_eq!(res, String("the function ran".to_string()));
}

#[test]
fn expression_function_unknown_function_is_reported_as_an_error() {
    let setup = Setup::new();

    let expr = ExpressionFunction{name: "unknown-fn".to_string(), arguments: vec!()};

    let context = setup.context();
    expr.evaluate(&context);
    // TODO: report errors better
}

#[test]
fn expression_math_does_basic_math() {
    let setup = Setup::new();

    let left  = box ExpressionLiteral{value: Number(10.0)};
    let right = box ExpressionLiteral{value: Number(5.0)};

    let expr = ExpressionMath::multiplication(left, right);

    let context = setup.context();
    let res = expr.evaluate(&context);

    assert_eq!(res, Number(50.0));
}

#[test]
fn expression_step_numeric_predicate_selects_that_node() {
    let setup = Setup::new();

    let input_node_1 = setup.doc.new_element("one".to_string());
    let input_node_2 = setup.doc.new_element("two".to_string());
    let mut input_nodeset = Nodeset::new();
    input_nodeset.add(input_node_1.clone());
    input_nodeset.add(input_node_2);

    let selected_nodes = box ExpressionLiteral{value: Nodes(input_nodeset)};
    let predicate = box ExpressionLiteral{value: Number(1.0)};

    let expr = ExpressionPredicate{node_selector: selected_nodes, predicate: predicate};

    let context = setup.context();
    let res = expr.evaluate(&context);

    let mut expected = Nodeset::new();
    expected.add(input_node_1);

    assert_eq!(res, Nodes(expected));
}

#[test]
fn expression_step_false_predicate_selects_no_nodes() {
    let setup = Setup::new();

    let input_node_1 = setup.doc.new_element("one".to_string());
    let input_node_2 = setup.doc.new_element("two".to_string());
    let mut input_nodeset = Nodeset::new();
    input_nodeset.add(input_node_1);
    input_nodeset.add(input_node_2);

    let selected_nodes = box ExpressionLiteral{value: Nodes(input_nodeset)};
    let predicate = box ExpressionLiteral{value: Boolean(false)};

    let expr = ExpressionPredicate{node_selector: selected_nodes, predicate: predicate};

    let context = setup.context();
    let res = expr.evaluate(&context);

    let expected = Nodeset::new();
    assert_eq!(res, Nodes(expected));
}

#[test]
fn expression_relational_does_basic_comparisons() {
    let setup = Setup::new();

    let left  = box ExpressionLiteral{value: Number(10.0)};
    let right = box ExpressionLiteral{value: Number(5.0)};

    let expr = ExpressionRelational::less_than(left, right);

    let context = setup.context();
    let res = expr.evaluate(&context);
    assert_eq!(res, Boolean(false));
}

#[test]
fn expression_root_node_finds_the_root() {
    let setup = Setup::new();

    let expr = box ExpressionRootNode;

    let context = setup.context();
    let res = expr.evaluate(&context);

    let mut expected = Nodeset::new();
    expected.add(setup.doc.root());

    assert_eq!(res, Nodes(expected));
}
