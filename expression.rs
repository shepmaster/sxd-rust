extern crate document;

use document::Nodeset;

use super::XPathEvaluationContext;
use super::XPathNodeTest;
use super::XPathValue;
use super::{Boolean,Number,String,Nodes};

use super::axis::XPathAxis;

pub trait XPathExpression<'n> {
    fn evaluate(& self, context: &XPathEvaluationContext<'n>) -> XPathValue;
}

pub struct ExpressionAnd<'n> {
    pub left:  Box<XPathExpression<'n>>,
    pub right: Box<XPathExpression<'n>>,
}

impl<'n> XPathExpression<'n> for ExpressionAnd<'n> {
    fn evaluate(& self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        Boolean(self.left.evaluate(context).boolean() &&
                self.right.evaluate(context).boolean())
    }
}

pub struct ExpressionContextNode;

impl<'n> XPathExpression<'n> for ExpressionContextNode {
    fn evaluate(&self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        let mut result = Nodeset::new();
        result.add(context.node().clone());
        Nodes(result.clone())
    }
}

pub struct ExpressionEqual<'n> {
    pub left:  Box<XPathExpression<'n>>,
    pub right: Box<XPathExpression<'n>>,
}

impl<'n> ExpressionEqual<'n> {
    fn boolean_evaluate(& self, context: &XPathEvaluationContext<'n>) -> bool {
        let left_val = self.left.evaluate(context);
        let right_val = self.right.evaluate(context);

        match (&left_val, &right_val) {
            (&Boolean(_), _) |
            (_, &Boolean(_)) => left_val.boolean() == right_val.boolean(),
            (&Number(_), _) |
            (_, &Number(_)) => left_val.number() == right_val.number(),
            _ => left_val.string() == right_val.string()
        }
    }
}

impl<'n> XPathExpression<'n> for ExpressionEqual<'n> {
    fn evaluate(& self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        Boolean(self.boolean_evaluate(context))
    }
}

pub struct ExpressionNotEqual<'n> {
    equal: ExpressionEqual<'n>,
}

impl<'n> ExpressionNotEqual<'n> {
    pub fn new(left: Box<XPathExpression<'n>>, right: Box<XPathExpression<'n>>) -> ExpressionNotEqual<'n> {
        ExpressionNotEqual {
            equal: ExpressionEqual{left: left, right: right}
        }
    }
}

impl<'n> XPathExpression<'n> for ExpressionNotEqual<'n> {
    fn evaluate(& self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        Boolean(!self.equal.boolean_evaluate(context))
    }
}

pub struct ExpressionFunction<'n> {
    pub name: String,
    pub arguments: Vec<Box<XPathExpression<'n>>>,
}

impl<'n> XPathExpression<'n> for ExpressionFunction<'n> {
    fn evaluate(& self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        match context.function_for_name(self.name.as_slice()) {
            Some(fun) => {
                // TODO: Error when argument count mismatch
                let args = self.arguments.iter().map(|ref arg| arg.evaluate(context)).collect();

                fun.evaluate(context, args)
            },
            None => fail!("throw UnknownXPathFunctionException(_name)"),
        }
    }
}

pub struct ExpressionLiteral {
    pub value: XPathValue,
}

impl<'n> XPathExpression<'n> for ExpressionLiteral {
    fn evaluate(& self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        self.value.clone()
    }
}

pub struct ExpressionMath<'n> {
    left:  Box<XPathExpression<'n>>,
    right: Box<XPathExpression<'n>>,
    operation: fn(f64, f64) -> f64,
}

fn      add(a: f64, b: f64) -> f64 {a + b}
fn subtract(a: f64, b: f64) -> f64 {a - b}
fn multiply(a: f64, b: f64) -> f64 {a * b}
fn   divide(a: f64, b: f64) -> f64 {a / b}
fn  modulus(a: f64, b: f64) -> f64 {a % b}

impl<'n> ExpressionMath<'n> {
    pub fn addition(left: Box<XPathExpression<'n>>, right: Box<XPathExpression<'n>>) -> ExpressionMath<'n> {
        ExpressionMath{left: left, right: right, operation: add}
    }

    pub fn subtraction(left: Box<XPathExpression<'n>>, right: Box<XPathExpression<'n>>) -> ExpressionMath<'n> {
        ExpressionMath{left: left, right: right, operation: subtract}
    }

    pub fn multiplication(left: Box<XPathExpression<'n>>, right: Box<XPathExpression<'n>>) -> ExpressionMath<'n> {
        ExpressionMath{left: left, right: right, operation: multiply}
    }

    pub fn division(left: Box<XPathExpression<'n>>, right: Box<XPathExpression<'n>>) -> ExpressionMath<'n> {
        ExpressionMath{left: left, right: right, operation: divide}
    }

    pub fn remainder(left: Box<XPathExpression<'n>>, right: Box<XPathExpression<'n>>) -> ExpressionMath<'n> {
        ExpressionMath{left: left, right: right, operation: modulus}
    }
}

impl<'n> XPathExpression<'n> for ExpressionMath<'n> {
    fn evaluate(& self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        let left = self.left.evaluate(context);
        let right = self.right.evaluate(context);
        let op = self.operation;
        return Number(op(left.number(), right.number()));
    }
}

pub struct ExpressionNegation<'n> {
    expression: Box<XPathExpression<'n>>,
}

impl<'n> XPathExpression<'n> for ExpressionNegation<'n> {
    fn evaluate(& self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        let result = self.expression.evaluate(context);
        return Number(-result.number());
    }
}

pub struct ExpressionOr<'n> {
    left:  Box<XPathExpression<'n>>,
    right: Box<XPathExpression<'n>>,
}

impl<'n> XPathExpression<'n> for ExpressionOr<'n> {
    fn evaluate(& self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        return Boolean(self.left.evaluate(context).boolean() ||
                       self.right.evaluate(context).boolean())
    }
}

pub struct ExpressionPath<'n> {
    start_point: Box<XPathExpression<'n>>,
    steps: Vec<Box<XPathExpression<'n>>>,
}

impl<'n> XPathExpression<'n> for ExpressionPath<'n> {
    fn evaluate(& self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        let mut result = self.start_point.evaluate(context).nodeset();

        for step in self.steps.iter() {
            let mut step_result = Nodeset::new();

            let mut sub_context = context.new_context_for(result.size());

            for current_node in result.iter() {
                sub_context.next(current_node.clone());
                let selected = step.evaluate(&sub_context);
                // TODO: What if it is not a nodeset?
                step_result.add_nodeset(&selected.nodeset());
            }

            result = step_result;
        }

        Nodes(result)
    }
}

pub struct ExpressionPredicate<'n> {
    pub node_selector: Box<XPathExpression<'n>>,
    pub predicate: Box<XPathExpression<'n>>,
}

impl<'n> ExpressionPredicate<'n> {
    fn include(value: &XPathValue, context: &XPathEvaluationContext) -> bool {
        match value {
            &Number(v) => context.position() == v as uint,
            _ => value.boolean()
        }
    }
}

impl<'n> XPathExpression<'n> for ExpressionPredicate<'n> {
    fn evaluate(&self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        let mut selected = Nodeset::new();

        let nodes = self.node_selector.evaluate(context).nodeset();

        let mut sub_context = context.new_context_for(nodes.size());

        for current_node in nodes.iter() {
            sub_context.next(current_node.clone());

            let value = self.predicate.evaluate(&sub_context);

            if ExpressionPredicate::include(&value, &sub_context) {
                selected.add(current_node.clone());
            }
        }

        Nodes(selected)
    }
}

pub struct ExpressionRelational<'n> {
    pub  left: Box<XPathExpression<'n>>,
    pub right: Box<XPathExpression<'n>>,
    pub operation: fn(f64, f64) -> bool,
}

fn             less_than(left: f64, right: f64) -> bool { left <  right }
fn    less_than_or_equal(left: f64, right: f64) -> bool { left <= right }
fn          greater_than(left: f64, right: f64) -> bool { left >  right }
fn greater_than_or_equal(left: f64, right: f64) -> bool { left >= right }

impl<'n> ExpressionRelational<'n> {
    pub fn less_than(left: Box<XPathExpression<'n>>,
                right: Box<XPathExpression<'n>>) -> ExpressionRelational<'n>
    {
        ExpressionRelational{left: left, right: right, operation: less_than}
    }

    pub fn less_than_or_equal(left: Box<XPathExpression<'n>>,
                       right: Box<XPathExpression<'n>>) -> ExpressionRelational<'n>
    {
        ExpressionRelational{left: left, right: right, operation: less_than_or_equal}
    }

    pub fn greater_than(left: Box<XPathExpression<'n>>,
                   right: Box<XPathExpression<'n>>) -> ExpressionRelational<'n>
    {
        ExpressionRelational{left: left, right: right, operation: greater_than}
    }

    pub fn greater_than_or_equal(left: Box<XPathExpression<'n>>,
                          right: Box<XPathExpression<'n>>) -> ExpressionRelational<'n>
    {
        ExpressionRelational{left: left, right: right, operation: greater_than_or_equal}
    }
}

impl<'n> XPathExpression<'n> for ExpressionRelational<'n> {
    fn evaluate(&self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        let left_val = self.left.evaluate(context);
        let right_val = self.right.evaluate(context);
        let op = self.operation;
        Boolean(op(left_val.number(), right_val.number()))
    }
}

pub struct ExpressionRootNode;

impl<'n> XPathExpression<'n> for ExpressionRootNode {
    fn evaluate(&self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        let n = &context.node;

        let mut result = Nodeset::new();
        result.add(n.document().root());
        Nodes(result)
    }
}

pub struct ExpressionStep {
    axis: Box<XPathAxis>,
    node_test: Box<XPathNodeTest>,
}

impl ExpressionStep {
    pub fn new(axis: Box<XPathAxis>, node_test: Box<XPathNodeTest>) -> ExpressionStep {
        ExpressionStep {axis: axis, node_test: node_test}
    }
}

impl<'n> XPathExpression<'n> for ExpressionStep {
    fn evaluate(&self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        let mut result = Nodeset::new();
        self.axis.select_nodes(context, self.node_test, & mut result);
        Nodes(result)
    }
}

pub struct ExpressionUnion<'n> {
    pub left:  Box<XPathExpression<'n>>,
    pub right: Box<XPathExpression<'n>>,
}

impl<'n> XPathExpression<'n> for ExpressionUnion<'n> {
    fn evaluate(&self, context: &XPathEvaluationContext<'n>) -> XPathValue {
        let mut left_val = self.left.evaluate(context).nodeset();
        let right_val = self.right.evaluate(context).nodeset();
        left_val.add_nodeset(&right_val);
        Nodes(left_val)
    }
}
