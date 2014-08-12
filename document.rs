#![crate_name = "document"]

use std::collections::hashmap::HashMap;
use std::collections::hashmap::HashSet;

#[deriving(Show)]
pub struct Document {
    /// Storage for each maintained type
    elements: Vec<Element>,
    attributes: Vec<Attribute>,

    // Primary associations between nodes
    children: HashMap<Parent, Vec<Child>>,
    parents: HashMap<Child, Parent>,

    // Associated attributes
    assigned_attributes: HashMap<ElementNode, HashSet<AttributeNode>>,
}

#[deriving(Show,Eq,PartialEq,Hash,Clone)]
pub struct ElementNode   { i: uint }
#[deriving(Show,Eq,PartialEq,Hash,Clone)]
pub struct AttributeNode { i: uint }

#[deriving(Show,Eq,PartialEq,Hash,Clone)]
pub enum Parent {
    ElementParent(ElementNode),
}

impl Parent {
    pub fn element(&self) -> Option<ElementNode> {
        match self {
            &ElementParent(e) => Some(e),
        }
    }
}

pub trait ToParent {
    fn to_parent(&self) -> Parent;
}

impl ToParent for ElementNode {
    fn to_parent(&self) -> Parent { ElementParent(*self) }
}

#[deriving(Show,Eq,PartialEq,Hash,Clone)]
pub enum Child {
    ElementChild(ElementNode),
}

impl Child {
    pub fn element(&self) -> Option<ElementNode> {
        match self {
            &ElementChild(e) => Some(e),
        }
    }
}

pub trait ToChild {
    fn to_child(&self) -> Child;
}

impl ToChild for ElementNode {
    fn to_child(&self) -> Child { ElementChild(*self) }
}

impl Document {
    pub fn new() -> Document {
        Document {
            elements: Vec::new(),
            attributes: Vec::new(),
            children: HashMap::new(),
            parents: HashMap::new(),
            assigned_attributes: HashMap::new(),
        }
    }

    fn next_element_ref(& self) -> ElementNode {
        ElementNode{i: self.elements.len()}
    }

    fn next_attribute_ref(& self) -> AttributeNode {
        AttributeNode{i: self.attributes.len()}
    }

    pub fn new_element(&mut self, name: &str) -> ElementNode {
        let eref = self.next_element_ref();
        self.elements.push(Element {
            name: name.to_string(),
        });
        eref
    }

    fn new_attribute(&mut self, name: &str, value: &str) -> AttributeNode {
        let aref = self.next_attribute_ref();
        self.attributes.push(Attribute {
            name: name.to_string(),
            value: value.to_string(),
        });
        aref
    }

    pub fn element<'a>(&'a self, element: ElementNode) -> &'a Element {
        &self.elements[element.i]
    }

    pub fn mut_element<'a>(&'a mut self, element: ElementNode) -> &'a mut Element {
        self.elements.get_mut(element.i)
    }

    fn attribute<'a>(&'a self, attribute: AttributeNode) -> &'a Attribute {
        &self.attributes[attribute.i]
    }

    fn mut_attribute<'a>(&'a mut self, attribute: AttributeNode) -> &'a mut Attribute {
        self.attributes.get_mut(attribute.i)
    }

    fn attribute_for(&self, element: ElementNode, name: &str) -> Option<AttributeNode> {
        match self.assigned_attributes.find(&element) {
            Some(ref attributes) => {
                let mut node_attrs = attributes.iter().map(|node| (node, self.attribute(*node)));
                match node_attrs.find(|&(_, attr)| attr.name.as_slice() == name) {
                    Some((node, _)) => Some(*node),
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn get_attribute(&self, element: ElementNode, name: &str) -> Option<&str> {
        match self.attribute_for(element, name) {
            Some(aref) => Some(self.attribute(aref).value.as_slice()),
            None => None,
        }
    }

    pub fn set_attribute(&mut self, element: ElementNode, name: &str, value: &str) -> AttributeNode {
        match self.attribute_for(element, name) {
            Some(aref) => {
                self.mut_attribute(aref).value = value.to_string();
                aref
            },
            None => {
                let aref = self.new_attribute(name, value);
                let assigned_attributes = self.assigned_attributes.find_or_insert(element, HashSet::new());
                assigned_attributes.insert(aref);
                aref
            },
        }
    }

    fn remove_parentage(&mut self, child: Child) {
        match self.parent(child) {
            Some(ref parent) => {
                match self.children.find_mut(parent) {
                    Some(children) => {
                        match children.as_slice().position_elem(&child) {
                            Some(idx) => { children.remove(idx); },
                            None => {},
                        }
                    },
                    None => {},
                }
            },
            None => {}
        }
    }

    pub fn append_child<P: ToParent, C: ToChild>(&mut self, parent: P, child: C) {
        let parent = parent.to_parent();
        let child = child.to_child();

        {
            let kids = self.children.find_or_insert(parent, Vec::new());
            kids.push(child);
        }

        self.remove_parentage(child);
        self.parents.insert(child, parent);
    }

    pub fn children<P: ToParent>(&self, parent: P) -> Vec<Child> {
        let parent = parent.to_parent();

        match self.children.find(&parent) {
            Some(v) => v.iter().map(|r| r.clone()).collect(),
            None => vec![],
        }
    }

    pub fn parent(&self, child: Child) -> Option<Parent> {
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

    let child_elem = children[0].element().unwrap();
    let result = d.element(child_elem);
    assert_eq!(result.name(), "beta");
}

#[test]
fn children_are_ordered() {
    let mut d = Document::new();
    let greek = d.new_element("greek");
    let alpha = d.new_element("alpha");
    let omega = d.new_element("omega");

    d.append_child(greek, alpha);
    d.append_child(greek, omega);
    let children = d.children(greek);

    assert_eq!(2, children.len());
    let child_elem1 = children[0].element().unwrap();
    let child_elem2 = children[1].element().unwrap();
    assert_eq!(d.element(child_elem1).name(), "alpha");
    assert_eq!(d.element(child_elem2).name(), "omega");
}

#[test]
fn children_know_their_parent() {
    let mut d = Document::new();
    let alpha = d.new_element("alpha");
    let beta  = d.new_element("beta");

    d.append_child(alpha, beta);

    let child = d.children(alpha)[0];
    let parent = d.parent(child).unwrap();

    let parent_elem = parent.element().unwrap();
    let result = d.element(parent_elem);
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

#[test]
fn attributes_can_be_reset() {
    let mut d = Document::new();
    let alpha = d.new_element("alpha");

    d.set_attribute(alpha, "hello", "world");
    d.set_attribute(alpha, "hello", "universe");

    let val = d.get_attribute(alpha, "hello").unwrap();
    assert_eq!(val, "universe");
}
