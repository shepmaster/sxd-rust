extern crate document;

use document::Nodeset;
use document::{Any,ElementAny,AttributeAny,TextAny};
use document::{Parent,ElementParent};
use document::{ElementChild};

use super::XPathEvaluationContext;
use super::XPathNodeTest;

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
        let d = context.document;
        let n = context.node;

        match n {
            ElementAny(e) => {
                for attr in d.attributes(e).iter() {
                    let mut attr_context = context.new_context_for(1);
                    attr_context.next(*attr);

                    node_test.test(&attr_context, result);
                }
            },
            _ => {}
        }
    }

    fn principal_node_type() -> PrincipalNodeType {
        Attribute
    }
}

fn maybe_parent(node: Any) -> Option<Parent> {
    match node {
        ElementAny(e) => Some(ElementParent(e)),
        AttributeAny(_) |
        TextAny(_) => None,
    }
}

pub struct AxisChild;

impl XPathAxis for AxisChild {
    fn select_nodes(&self,
                    context:   &XPathEvaluationContext,
                    node_test: &XPathNodeTest,
                    result:    &mut Nodeset)
    {
        let d = context.document;
        let n = context.node;

        match maybe_parent(n) {
            Some(parent) => {
                for child in d.children(parent).iter() {
                    let mut child_context = context.new_context_for(1);
                    child_context.next(*child);

                    node_test.test(&child_context, result);
                }
            },
            None => {}
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
        let d = context.document;
        let n = context.node;

        match maybe_parent(n) {
            Some(parent) => {
                for child in d.children(parent).iter() {
                    let mut child_context = context.new_context_for(1);
                    child_context.next(*child);

                    node_test.test(&child_context, result);
                    self.select_nodes(&child_context, node_test, result);
                }
            },
            None => {}
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
        let d = context.document;
        let n = context.node;

        // Comments and text nodes and PIs all live in the
        // parent<->children section, but attributes are special and
        // need something special.

        let parent = match n {
            ElementAny(e) => d.parent(e),
            TextAny(e) => d.parent(e),
            AttributeAny(a) => d.attribute_parent(a).map(|e| ElementParent(e)),
        };

        match parent {
            Some(p) => {
                let mut parent_context = context.new_context_for(1);
                parent_context.next(p);
                node_test.test(&parent_context, result);
            },
            None => {}
        }
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
