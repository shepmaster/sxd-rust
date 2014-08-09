use super::XPathEvaluationContext;
use super::XPathValue;

trait XPathExpression {
    fn evaluate(context: &XPathEvaluationContext) -> XPathValue;
}
