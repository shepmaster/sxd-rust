use super::super::XPathEvaluationContext;
use super::super::XPathNodeTest;
use super::super::Nodeset;
use super::XPathAxis;

pub struct AxisDescendant;

impl XPathAxis for AxisDescendant {
    fn select_nodes(&self,
                    context:   &XPathEvaluationContext,
                    node_test: &XPathNodeTest,
                    result:    &mut Nodeset)
    {
        for child in context.node().children() {
            let child_context = context.new_context_for(1);
            child_context.next(child);

            node_test.test(&child_context, result);
            self.select_nodes(&child_context, node_test, result);
        }
    }
}
