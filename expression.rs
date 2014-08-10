use super::XPathEvaluationContext;
use super::XPathValue;
use super::{Boolean,Number,String,Nodes};
use super::Nodeset;

pub trait XPathExpression {
    fn evaluate(&self, context: &XPathEvaluationContext) -> XPathValue;
}

pub struct ExpressionAnd {
    pub left:  Box<XPathExpression>,
    pub right: Box<XPathExpression>,
}

impl XPathExpression for ExpressionAnd {
    fn evaluate(&self, context: &XPathEvaluationContext) -> XPathValue {
        Boolean(self.left.evaluate(context).boolean() &&
                self.right.evaluate(context).boolean())
    }
}

pub struct ExpressionContextNode;

impl XPathExpression for ExpressionContextNode {
    fn evaluate(&self, context: &XPathEvaluationContext) -> XPathValue {
        let mut result = Nodeset;
        result.add(context.node());
        Nodes(result)
    }
}

pub struct ExpressionEqual {
    pub left:  Box<XPathExpression>,
    pub right: Box<XPathExpression>,
}

impl ExpressionEqual {
    fn boolean_evaluate(&self, context: &XPathEvaluationContext) -> bool {
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

impl XPathExpression for ExpressionEqual {
    fn evaluate(&self, context: &XPathEvaluationContext) -> XPathValue {
        Boolean(self.boolean_evaluate(context))
    }
}

pub struct ExpressionNotEqual {
    equal: ExpressionEqual,
}

impl ExpressionNotEqual {
    pub fn new(left: Box<XPathExpression>, right: Box<XPathExpression>) -> ExpressionNotEqual {
        ExpressionNotEqual {
            equal: ExpressionEqual{left: left, right: right}
        }
    }
}

impl XPathExpression for ExpressionNotEqual {
    fn evaluate(&self, context: &XPathEvaluationContext) -> XPathValue {
        Boolean(!self.equal.boolean_evaluate(context))
    }
}

pub struct ExpressionFunction {
    pub name: String,
    pub arguments: Vec<Box<XPathExpression>>,
}

impl XPathExpression for ExpressionFunction {
    fn evaluate(&self, context: &XPathEvaluationContext) -> XPathValue {
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

impl XPathExpression for ExpressionLiteral {
    fn evaluate(&self, context: &XPathEvaluationContext) -> XPathValue {
        self.value.clone()
    }
}

pub struct ExpressionMath {
    left:  Box<XPathExpression>,
    right: Box<XPathExpression>,
    operation: fn(f64, f64) -> f64,
}

fn      add(a: f64, b: f64) -> f64 {a + b}
fn subtract(a: f64, b: f64) -> f64 {a - b}
fn multiply(a: f64, b: f64) -> f64 {a * b}
fn   divide(a: f64, b: f64) -> f64 {a / b}
fn  modulus(a: f64, b: f64) -> f64 {a % b}

impl ExpressionMath {
    pub fn addition(left: Box<XPathExpression>, right: Box<XPathExpression>) -> ExpressionMath {
        ExpressionMath{left: left, right: right, operation: add}
    }

    pub fn subtraction(left: Box<XPathExpression>, right: Box<XPathExpression>) -> ExpressionMath {
        ExpressionMath{left: left, right: right, operation: subtract}
    }

    pub fn multiplication(left: Box<XPathExpression>, right: Box<XPathExpression>) -> ExpressionMath {
        ExpressionMath{left: left, right: right, operation: multiply}
    }

    pub fn division(left: Box<XPathExpression>, right: Box<XPathExpression>) -> ExpressionMath {
        ExpressionMath{left: left, right: right, operation: divide}
    }

    pub fn remainder(left: Box<XPathExpression>, right: Box<XPathExpression>) -> ExpressionMath {
        ExpressionMath{left: left, right: right, operation: modulus}
    }
}

impl XPathExpression for ExpressionMath {
    fn evaluate(&self, context: &XPathEvaluationContext) -> XPathValue {
        let left = self.left.evaluate(context);
        let right = self.right.evaluate(context);
        let op = self.operation;
        return Number(op(left.number(), right.number()));
    }
}

pub struct ExpressionNegation {
    expression: Box<XPathExpression>,
}

impl XPathExpression for ExpressionNegation {
    fn evaluate(&self, context: &XPathEvaluationContext) -> XPathValue {
        let result = self.expression.evaluate(context);
        return Number(-result.number());
    }
}
