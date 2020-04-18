use std::collections::hash_map::HashMap;
use std::fmt;

type AttrMap = HashMap<String, String>;

struct Node {
    // data common to all nodes:
    children: Vec<Node>,

    // data specific to each node type:
    node_type: NodeType,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({})", self.node_type)?;
        for child in self.children.iter() {
            writeln!(f, "|-{}", child)?;
        }
        Ok(())
    }
}

enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match &*self {
           NodeType::Text(text) => write!(f, "{}", text),
           NodeType::Element(element_data) => {
            write!(f, "{:?}", element_data)
           },
           NodeType::Comment(text) => write!(f, "{}", text),
       }
    }
}

#[derive(Debug)]
struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

fn create_text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

fn create_element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}

fn create_comment(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Comment(data),
    }
}
