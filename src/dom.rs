use std::collections::hash_map::HashMap;
use std::fmt;

type AttrMap = HashMap<String, String>;

pub struct Node {
    // data common to all nodes:
    children: Vec<Node>,

    // data specific to each node type:
    node_type: NodeType,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Node({})", self.node_type)?;
        for child in self.children.iter() {
            writeln!(f, "|-{}", child)?;
        }
        Ok(())
    }
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            NodeType::Text(text) => write!(f, "{}", text),
            NodeType::Element(element_data) => write!(f, "{:?}", element_data),
        }
    }
}

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub fn create_text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn create_element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}

pub fn get_css_text(dom_tree: Node) -> String {
    let nodes = dom_tree.children;
    // Find Element with style tag and get its first child
    for node in nodes.iter() {
        match &node.node_type {
            NodeType::Text(_) => {}
            NodeType::Element(el) => {
                if el.tag_name == "style" {
                    return get_node_text(&node.children[0]);
                }
            }
        }
    }

    String::from("")
}

fn get_node_text(node: &Node) -> String {
    match &node.node_type {
        NodeType::Text(s) => return s.to_string(),
        NodeType::Element(_) => return String::from(""),
    }
}
