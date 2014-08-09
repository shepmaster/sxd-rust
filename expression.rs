use super::XPathEvaluationContext;
use super::XPathValue;
use super::Boolean;

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
