#![crate_name = "xpath"]

use std::collections::HashMap;

#[deriving(Show,PartialEq,Clone)]
pub struct Node {
    parent: Option<uint>,
    children: Vec<Node>,
    attributes: Vec<Node>,
}

impl Node {
    pub fn new() -> Node {
        Node { parent: None, children: vec!(), attributes: vec!() }
    }

    pub fn new_with_parent(id: uint) -> Node {
        Node { parent: Some(id), children: vec!(), attributes: vec!() }
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
pub enum XPathValue<'n> {
    Boolean(bool),
    Number(f64),
    String(String),
    Nodes(Nodeset<'n>), // rename as Nodeset
}

impl<'n> XPathValue<'n> {
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

    fn nodeset(&self) -> Nodeset<'n> {
        match *self {
            Nodes(ref ns) => ns.clone(),
            _ => fail!("Did not evaluate to a nodeset!"),
        }
    }
}

pub trait XPathFunction<'n> {
    fn evaluate(&self,
                context: &XPathEvaluationContext,
                args: Vec<XPathValue>) -> XPathValue<'n>;
}

pub struct XPathEvaluationContext<'a> {
    pub node: & 'a Node,
    pub functions: & 'a HashMap<String, Box<XPathFunction<'a>>>,
    position: uint,
}

impl<'a> XPathEvaluationContext<'a> {
    pub fn new(node: & 'a Node,
               functions: & 'a HashMap<String, Box<XPathFunction<'a>>>) -> XPathEvaluationContext<'a>
    {
        XPathEvaluationContext {
            node: node,
            functions: functions,
            position: 0,
        }
    }

    fn node(&self) -> & 'a Node {
        self.node
    }

    fn new_context_for(& self, size: uint) -> XPathEvaluationContext<'a> {
        XPathEvaluationContext {
            node: self.node,
            functions: self.functions,
            position: 0,
        }
    }

    fn next(& mut self, node: &Node) {
        self.position += 1;
    }

    fn position(&self) -> uint {
        self.position
    }

    fn function_for_name(&self, name: &str) -> Option<& 'a Box<XPathFunction<'a>>> {
        self.functions.find(&name.to_string())
    }
}


pub struct XPathNodeTest;

impl XPathNodeTest {
    fn test(&self, context: &XPathEvaluationContext, result: &mut Nodeset) {
    }
}

#[deriving(Show,PartialEq,Clone)]
pub struct Nodeset<'n> {
    nodes: Vec<& 'n Node>,
}

impl<'n> Nodeset<'n> {
    pub fn new() -> Nodeset<'n> {
        Nodeset{nodes: Vec::new()}
    }

    pub fn add(& mut self, node: & 'n Node) {
        self.nodes.push(node);
    }

    fn add_nodeset(& mut self, nodes: &Nodeset<'n>) {
        self.nodes.push_all(nodes.nodes.as_slice());
    }

    pub fn size(&self) -> uint {
        self.nodes.len()
    }

    fn iter(&self) -> std::slice::Items<& 'n Node> {
        self.nodes.iter()
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
