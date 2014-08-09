use super::XPathEvaluationContext;
use super::XPathNodeTest;
use super::Nodeset;

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

pub struct AxisChild;

impl XPathAxis for AxisChild {
    fn select_nodes(&self,
                    context:   &XPathEvaluationContext,
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

pub struct AxisParent;

impl XPathAxis for AxisParent {
    fn select_nodes(&self,
                    context:   &XPathEvaluationContext,
                    node_test: &XPathNodeTest,
                    result:    &mut Nodeset)
    {
        let parent_context = context.new_context_for(1);
        parent_context.next(context.node().parent());

        node_test.test(&parent_context, result);
    }
}

pub struct AxisSelf;

impl XPathAxis for AxisSelf {
    fn select_nodes(&self,
                    context:   &XPathEvaluationContext,
                    node_test: &XPathNodeTest,
                    result:    &mut Nodeset)
    {
        node_test.test(context, result);
    }
}
