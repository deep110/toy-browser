/// CSS parser
///
/// Example CSS:
///
/// h1, h2, h3 { margin: auto; color: #cc0000; }
/// div.note { margin-bottom: 20px; padding: 10px; }
/// #answer { display: none; }
///
/// Each rule has selectors and declarations applied to it

use super::Parser;
use super::colors;

#[derive(Debug)]
pub struct Stylesheet {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    selectors: Vec<Selector>,       // h1, h2, h3
    declarations: Vec<Declaration>, // { margin: auto; color: #cc0000; }
}

#[derive(Debug)]
enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

#[derive(Debug)]
struct Declaration {
    name: String,
    value: Value,
}

#[derive(Debug)]
enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug)]
enum Unit {
    Px,
}

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new() -> Color {
        Color{r: 0, g: 0, b: 0, a: 255}
    }

    pub fn from(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color{r: r, g: g, b: b, a: a}
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.b == other.b && self.g == other.g && self.a == other.a
    }
}

pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser {
        pos: 0,
        input: source,
    };
    Stylesheet {
        rules: parse_rules(&mut parser),
    }
}

fn parse_rules(parser: &mut Parser) -> Vec<Rule> {
    let mut rules = Vec::new();

    loop {
        parser.skip_whitespace();
        if parser.eof() {
            break;
        }
        rules.push(parse_rule(parser));
    }

    return rules;
}

fn parse_rule(parser: &mut Parser) -> Rule {
    Rule {
        selectors: Vec::new(),
        declarations: Vec::new(),
    }
}

// fn parse_simple_selector(parser: &mut Parser) -> Selector {
    
// }

// fn parse_declaration(parser: &mut Parser) -> Declaration {
    
// }
