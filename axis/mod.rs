use super::XPathEvaluationContext;
use super::XPathNodeTest;
use super::Nodeset;

pub mod child;

enum PrincipalNodeType {
  Attribute,
  Element,
}

/// A directed traversal of Nodes.
trait XPathAxis {
    /// Applies the given node test to the nodes selected by this axis,
    /// adding matching nodes to the nodeset.
    fn select_nodes(context:   &XPathEvaluationContext,
                    node_test: &XPathNodeTest,
                    result:    &mut Nodeset);

    /// Describes what node type is naturally selected by this axis.
    fn principal_node_type() -> PrincipalNodeType {
        Element
    }
}
