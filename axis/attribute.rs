use super::super::XPathEvaluationContext;
use super::super::XPathNodeTest;
use super::super::Nodeset;
use super::XPathAxis;
use super::PrincipalNodeType;
use super::Attribute;

pub struct AxisAttribute;

impl XPathAxis for AxisAttribute {
    fn select_nodes(&self,
                    context:   &XPathEvaluationContext,
                    node_test: &XPathNodeTest,
                    result:    &mut Nodeset)
    {
        for attr in context.node().attributes() {
            let attr_context = context.new_context_for(1);
            attr_context.next(attr);

            node_test.test(&attr_context, result);
        }
    }

    fn principal_node_type() -> PrincipalNodeType {
        Attribute
    }
}
