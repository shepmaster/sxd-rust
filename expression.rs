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
