#![crate_name = "xpath"]

use std::collections::HashMap;

#[deriving(Show,PartialEq,Clone)]
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
}

impl<'a> XPathEvaluationContext<'a> {
    fn node(&self) -> & 'a Node {
        self.node
    }

    fn new_context_for(& self, size: uint) -> XPathEvaluationContext<'a> {
        XPathEvaluationContext {
            node: self.node,
            functions: self.functions,
        }
    }

    fn next(&self, node: &Node) {
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
    a: Vec<& 'n Node>,
}

impl<'n> Nodeset<'n> {
    fn new() -> Nodeset<'n> {
        Nodeset{a: Vec::new()}
    }

    fn add(& mut self, node: & 'n Node) {
        self.a.push(node);
    }

    fn add_nodeset(& mut self, nodes: &Nodeset<'n>) {
        self.a.push_all(nodes.a.as_slice());
    }

    fn size(&self) -> uint {
        0
    }

    fn iter(&self) -> EmptyIterator<& 'n Node> {
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
