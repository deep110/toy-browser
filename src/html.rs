// HTML parser
use std::collections::hash_map::HashMap;

use super::dom;

// parse html
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::create_element(String::from("html"), HashMap::new(), nodes)
    }
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    // Read the current character without consuming it.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Do the next characters start with the given string?
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    // skip characters until test fn returns true
    fn consume_while<F: Fn(char) -> bool>(&mut self, test: F) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn skip_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    // Parse a single node.
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::create_text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        // opening tag
        assert!('<' == self.consume_char());
        let tag = self.parse_tag_name();
        let attributes = self.parse_attributes();
        assert!('>' == self.consume_char());

        // parse nodes
        let children = self.parse_nodes();

        // closing tag
        assert!('<' == self.consume_char());
        assert!('/' == self.consume_char());
        assert!(self.parse_tag_name() == tag);
        assert!('>' == self.consume_char());

        return dom::create_element(tag, attributes, children);
    }

    fn parse_attr(&mut self) -> (String, String) {
        // parse name
        let name = self.consume_while(|c| c != '=');
        assert!(self.consume_char() == '=');
        // parse value
        let open_comma = self.consume_char(); // " or '
        let value = self.consume_while(|c| c != open_comma);
        assert!(self.consume_char() == open_comma);

        return (name, value);
    }

    fn parse_attributes(&mut self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();

        loop {
            self.skip_whitespace();
            if self.next_char() == '>' {
                break;
            }

            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }

        return attributes;
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.skip_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node())
        }
        return nodes;
    }
}
