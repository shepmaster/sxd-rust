#![crate_name = "xpath"]

use std::collections::HashMap;

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


#[deriving(PartialEq,Show,Clone)]
pub enum XPathValue {
    Boolean(bool),
    Number(f64),
    String(String),
    Nodes(Nodeset), // rename as Nodeset
}

impl XPathValue {
    fn boolean(&self) -> bool {
        match *self {
            Boolean(val) => val,
            Number(n) => n != 0.0 && ! n.is_nan(),
            String(ref s) => ! s.is_empty(),
            Nodes(ref nodeset) => nodeset.size() > 0,
        }
    }

    fn number(&self) -> f64 {
        match *self {
            Number(val) => val,
            _ => -42.0
        }
    }

    fn string(&self) -> String {
        match *self {
            String(ref val) => val.clone(),
            _ => "Unimplemented".to_string(),
        }
    }

    fn nodeset(&self) -> Nodeset {
        Nodeset::new()
    }
}

pub trait XPathFunction {
    fn evaluate(&self,
                context: &XPathEvaluationContext,
                args: Vec<XPathValue>) -> XPathValue;
}

pub struct XPathEvaluationContext<'a> {
    pub node: & 'a Node,
    pub functions: & 'a HashMap<String, Box<XPathFunction>>,
}

impl<'a> XPathEvaluationContext<'a> {
    fn node(&self) -> &Node {
        self.node
    }

    fn new_context_for(&self, size: uint) -> XPathEvaluationContext {
        XPathEvaluationContext {
            node: self.node,
            functions: self.functions,
        }
    }

    fn next(&self, node: &Node) {
    }

    fn function_for_name(&self, name: &str) -> Option<&Box<XPathFunction>> {
        self.functions.find(&name.to_string())
    }
}


pub struct XPathNodeTest;

impl XPathNodeTest {
    fn test(&self, context: &XPathEvaluationContext, result: &mut Nodeset) {
    }
}

#[deriving(Show,PartialEq,Clone)]
pub struct Nodeset {
    a: int,
}

impl Nodeset {
    fn new() -> Nodeset {
        Nodeset{a: 0}
    }

    fn add(&mut self, node: &Node) {
    }

    fn add_nodeset(&mut self, nodes: &Nodeset) {
    }

    fn size(&self) -> uint {
        0
    }

    fn iter(&self) -> EmptyIterator<&Node> {
        EmptyIterator
    }
}

struct EmptyIterator<T>;

impl<T> Iterator<T> for EmptyIterator<T> {
    fn next(&mut self) -> Option<T> { None }
}

pub mod token;
pub mod tokenizer;
pub mod deabbreviator;
pub mod disambiguator;
pub mod axis;
pub mod expression;
