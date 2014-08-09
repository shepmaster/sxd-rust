#![crate_name = "xpath"]

struct Node {
    children: Vec<Node>,
}

struct XPathEvaluationContext<'a> {
    node: & 'a Node,
}

struct XPathNodeTest;
struct Nodeset;

impl Node {
    fn children(&self) -> std::slice::Items<Node> {
        self.children.iter()
    }
}

impl<'a> XPathEvaluationContext<'a> {
    fn node(&self) -> &Node {
        self.node
    }

    fn new_context_for(&self, size: uint) -> XPathEvaluationContext {
        XPathEvaluationContext {
            node: self.node,
        }
    }

    fn next(&self, node: &Node) {
    }
}

impl XPathNodeTest {
    fn test(&self, context: &XPathEvaluationContext, result: &mut Nodeset) {
    }
}

pub mod token;
pub mod tokenizer;
pub mod deabbreviator;
pub mod disambiguator;
pub mod axis;
