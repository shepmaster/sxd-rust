use super::XPathEvaluationContext;
use super::XPathValue;
use super::Boolean;
use super::Nodeset;
use super::Nodes;

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
