use super::super::XPathEvaluationContext;
use super::super::XPathNodeTest;
use super::super::Nodeset;
use super::XPathAxis;

pub struct AxisChild;

impl XPathAxis for AxisChild {
    fn select_nodes(context:   &XPathEvaluationContext,
                    node_test: &XPathNodeTest,
                    result:    &mut Nodeset)
    {
        for child in context.node().children() {
            let child_context = context.new_context_for(1);
            child_context.next(child);

            node_test.test(&child_context, result);
        }
    }
}
