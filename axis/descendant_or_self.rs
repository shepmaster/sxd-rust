use super::super::XPathEvaluationContext;
use super::super::XPathNodeTest;
use super::super::Nodeset;
use super::XPathAxis;
use super::descendant::AxisDescendant;

pub struct AxisDescendantOrSelf {
    descendant: AxisDescendant,
}

impl XPathAxis for AxisDescendantOrSelf {
    fn select_nodes(&self,
                    context:   &XPathEvaluationContext,
                    node_test: &XPathNodeTest,
                    result:    &mut Nodeset)
    {
        node_test.test(context, result);
        self.descendant.select_nodes(context, node_test, result);
    }
}
