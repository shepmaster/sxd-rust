use super::XPathEvaluationContext;
use super::XPathNodeTest;
use super::Nodeset;

pub mod attribute;
pub mod child;
pub mod descendant;
pub mod descendant_or_self;

enum PrincipalNodeType {
  Attribute,
  Element,
}

/// A directed traversal of Nodes.
trait XPathAxis {
    /// Applies the given node test to the nodes selected by this axis,
    /// adding matching nodes to the nodeset.
    fn select_nodes(&self,
                    context:   &XPathEvaluationContext,
                    node_test: &XPathNodeTest,
                    result:    &mut Nodeset);

    /// Describes what node type is naturally selected by this axis.
    fn principal_node_type() -> PrincipalNodeType {
        Element
    }
}
