#![crate_name = "xpath"]

pub struct Node {
    children: Vec<Node>,
    attributes: Vec<Node>,
}

#[deriving(PartialEq,Show)]
pub enum XPathValue {
    Boolean(bool),
}

pub struct XPathEvaluationContext<'a> {
    pub node: & 'a Node,
}

pub struct XPathNodeTest;
pub struct Nodeset;

impl Node {
    pub fn new() -> Node {
        Node { children: vec!(), attributes: vec!() }
    }

    fn parent(&self) -> &Node {
        self
    }

    fn children(&self) -> std::slice::Items<Node> {
        self.children.iter()
    }

    fn attributes(&self) -> std::slice::Items<Node> {
        self.attributes.iter()
    }
}

impl XPathValue {
    fn boolean(&self) -> bool {
        match *self {
            Boolean(val) => val,
        }
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
pub mod expression;
