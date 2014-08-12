#![crate_name = "document"]

use std::collections::hashmap::HashMap;
use std::collections::hashmap::HashSet;

#[deriving(Show)]
pub struct Document {
    /// Storage for each maintained type
    elements: Vec<Element>,
    attributes: Vec<Attribute>,

    // Primary associations between nodes
    children: HashMap<Node, HashSet<Node>>, // Needs order!
    parents: HashMap<Node, Node>,

    // Associated attributes
    attrs: HashMap<Node, HashSet<Node>>,
}

#[deriving(Show,Eq,PartialEq,Hash,Clone)]
pub enum Node {
    ElementNode(uint),
    AttributeNode(uint),
}

impl Document {
    pub fn new() -> Document {
        Document {
            elements: Vec::new(),
            attributes: Vec::new(),
            children: HashMap::new(),
            parents: HashMap::new(),
            attrs: HashMap::new(),
        }
    }

    fn next_element_ref(& self) -> Node {
        ElementNode(self.elements.len())
    }

    pub fn new_element(&mut self, name: &str) -> Node {
        let eref = self.next_element_ref();
        self.elements.push(Element {
            name: name.to_string(),
        });
        eref
    }

    pub fn element<'a>(&'a self, element: Node) -> &'a Element {
        match element {
            ElementNode(i) => &self.elements[i],
            _ => fail!("Not an element"),
        }
    }

    pub fn mut_element<'a>(&'a mut self, element: Node) -> &'a mut Element {
        match element {
            ElementNode(i) => self.elements.get_mut(i),
            _ => fail!("Not an element"),
        }
    }

    fn next_attribute_ref(& self) -> Node {
        AttributeNode(self.attributes.len())
    }

    pub fn attribute<'a>(&'a self, attribute: Node) -> &'a Attribute {
        match attribute {
            AttributeNode(i) => &self.attributes[i],
            _ => fail!("Not an attribute"),
        }
    }

    pub fn get_attribute(&self, element: Node, name: &str) -> Option<&str> {
        match self.attrs.find(&element) {
            Some(ref attrs) => {
                let mut real_attrs = attrs.iter().map(|node| self.attribute(*node));
                match real_attrs.find(|attr| attr.name.as_slice() == name) {
                    Some(ref attr) => Some(attr.value.as_slice()),
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn set_attribute(&mut self, element: Node, name: &str, value: &str) -> Node {

        // modify existing first

        let aref = self.next_attribute_ref();
        self.attributes.push(Attribute {
            name: name.to_string(),
            value: value.to_string(),
        });

        let attrs = self.attrs.find_or_insert(element, HashSet::new());
        attrs.insert(aref);

        aref
    }

    fn remove_parentage(&mut self, child: Node) {
        match self.parent(child) {
            Some(ref parent) => {
                match self.children.find_mut(parent) {
                    Some(children) => { children.remove(&child); },
                    None => {},
                }
            },
            None => {}
        }
    }

    pub fn append_child(&mut self, parent: Node, child: Node) {
        {
            let kids = self.children.find_or_insert(parent, HashSet::new());
            kids.insert(child);
        }

        self.remove_parentage(child);
        self.parents.insert(child, parent);
    }

    pub fn children(&self, parent: Node) -> Vec<Node> {
        match self.children.find(&parent) {
            Some(v) => v.iter().map(|r| r.clone()).collect(),
            None => vec![],
        }
    }

    pub fn parent(&self, child: Node) -> Option<Node> {
        self.parents.find_copy(&child)
    }
}

#[deriving(Show)]
pub struct Element {
    name: String,
}

impl Element {
    pub fn name(&self) -> &str {
        self.name.as_slice()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

#[deriving(Show)]
pub struct Attribute {
    name: String,
    value: String,
}

pub struct Text;
pub struct Comment;
pub struct Root;

#[test]
fn can_add_an_element_as_a_child() {
    let mut d = Document::new();
    let alpha = d.new_element("alpha");
    let beta  = d.new_element("beta");

    d.append_child(alpha, beta);

    let children = d.children(alpha);
    assert_eq!(1, children.len());

    let result = d.element(children[0]);
    assert_eq!(result.name(), "beta");
}

#[test]
fn children_know_their_parent() {
    let mut d = Document::new();
    let alpha = d.new_element("alpha");
    let beta  = d.new_element("beta");

    d.append_child(alpha, beta);

    let child = d.children(alpha)[0];
    let parent = d.parent(child).unwrap();

    let result = d.element(parent);
    assert_eq!(result.name(), "alpha");
}

#[test]
fn replacing_parent_updates_original_parent() {
    let mut d = Document::new();
    let parent1 = d.new_element("parent1");
    let parent2 = d.new_element("parent2");
    let child = d.new_element("child");

    d.append_child(parent1, child);
    d.append_child(parent2, child);

    assert!(d.children(parent1).is_empty());
    assert_eq!(1, d.children(parent2).len());
}

#[test]
fn can_rename_an_element() {
    let mut d = Document::new();
    let alpha = d.new_element("alpha");

    {
        let element = d.mut_element(alpha);
        element.set_name("beta");
    }

    let beta = d.element(alpha);
    assert_eq!(beta.name(), "beta");
}

#[test]
fn elements_have_attributes() {
    let mut d = Document::new();
    let alpha = d.new_element("alpha");

    d.set_attribute(alpha, "hello", "world");
    let val = d.get_attribute(alpha, "hello").unwrap();
    assert_eq!(val, "world");
}
