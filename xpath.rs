#![crate_name = "xpath"]

pub struct Node {
    children: Vec<Node>,
    attributes: Vec<Node>,
}

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


#[deriving(PartialEq,Show)]
pub enum XPathValue {
    Boolean(bool),
    Nodes(Nodeset), // rename as Nodeset
}

impl XPathValue {
    fn boolean(&self) -> bool {
        match *self {
            Boolean(val) => val,
            Nodes(nodeset) => nodeset.size() > 0,
        }
    }
}

pub struct XPathEvaluationContext<'a> {
    pub node: & 'a Node,
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


pub struct XPathNodeTest;

impl XPathNodeTest {
    fn test(&self, context: &XPathEvaluationContext, result: &mut Nodeset) {
    }
}

#[deriving(Show,PartialEq)]
pub struct Nodeset;

impl Nodeset {
    fn add(&mut self, node: &Node) {
    }

    fn size(&self) -> uint {
        0
    }
}


pub mod token;
pub mod tokenizer;
pub mod deabbreviator;
pub mod disambiguator;
pub mod axis;
pub mod expression;
