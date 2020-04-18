// HTML parser
use std::collections::hash_map::HashMap;

use super::dom;
use super::Parser;

// parse html
pub fn parse(source: String) -> dom::Node {
    let mut parser = Parser {
        pos: 0,
        input: source,
    };
    let mut nodes = parse_nodes(&mut parser);

    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::create_element(String::from("html"), HashMap::new(), nodes)
    }
}

fn parse_nodes(parser: &mut Parser) -> Vec<dom::Node> {
    let mut nodes = Vec::new();
    loop {
        parser.skip_whitespace();
        if parser.eof() || parser.starts_with("</") {
            break;
        }
        nodes.push(parse_node(parser))
    }
    return nodes;
}

// Parse a single node.
fn parse_node(parser: &mut Parser) -> dom::Node {
    match parser.next_char() {
        '<' => parse_element(parser),
        _ => parse_text(parser),
    }
}

fn parse_text(parser: &mut Parser) -> dom::Node {
    dom::create_text(parser.consume_while(|c| c != '<'))
}

fn parse_element(parser: &mut Parser) -> dom::Node {
    // opening tag
    assert!('<' == parser.consume_char());
    let tag = parser.parse_tag_name();
    let attributes = parse_attributes(parser);
    assert!('>' == parser.consume_char());

    // parse nodes
    let children = parse_nodes(parser);

    // closing tag
    assert!('<' == parser.consume_char());
    assert!('/' == parser.consume_char());
    assert!(parser.parse_tag_name() == tag);
    assert!('>' == parser.consume_char());

    return dom::create_element(tag, attributes, children);
}

fn parse_attributes(parser: &mut Parser) -> HashMap<String, String> {
    let mut attributes = HashMap::new();

    loop {
        parser.skip_whitespace();
        if parser.next_char() == '>' {
            break;
        }

        let (name, value) = parse_attr(parser);
        attributes.insert(name, value);
    }

    return attributes;
}

fn parse_attr(parser: &mut Parser) -> (String, String) {
    // parse name
    let name = parser.consume_while(|c| c != '=');
    assert!(parser.consume_char() == '=');
    // parse value
    let open_comma = parser.consume_char(); // " or '
    let value = parser.consume_while(|c| c != open_comma);
    assert!(parser.consume_char() == open_comma);

    return (name, value);
}
