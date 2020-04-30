use super::colors;
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
    Number(f32),
}

#[derive(Debug)]
enum Unit {
    Px,
    Em,
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
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn from(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.b == other.b && self.g == other.g && self.a == other.a
    }
}

/// Decides the order in which to apply css properties
///
/// For example, id takes preference over class
pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
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

// Parse a rule set: `<selectors> { <declarations> }`
fn parse_rule(parser: &mut Parser) -> Rule {
    Rule {
        selectors: parse_selectors(parser),
        declarations: parse_declarations(parser),
    }
}

fn parse_selectors(parser: &mut Parser) -> Vec<Selector> {
    let mut selectors = Vec::new();

    loop {
        selectors.push(Selector::Simple(parse_simple_selector(parser)));
        parser.skip_whitespace();
        match parser.next_char() {
            ',' => {
                parser.consume_char();
                parser.skip_whitespace();
            }
            '{' => break, // start of declarations
            c => panic!("unexpected character in selector parsing: `{}`", c),
        }
    }

    // Return selectors with highest specificity first, for use in matching.
    selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));

    return selectors;
}

fn parse_declarations(parser: &mut Parser) -> Vec<Declaration> {
    let mut declarations = Vec::new();

    assert_eq!('{', parser.consume_char()); // start of declaration

    loop {
        parser.skip_whitespace();
        if parser.next_char() == '}' {
            break; // end of declaration
        }
        declarations.push(parse_declaration(parser));
        if parser.next_char() == ';' {
            parser.consume_char();
        }
    }

    assert_eq!('}', parser.consume_char()); // end of declaration

    return declarations;
}

// selector of format => type#id.class1.class2.class3
fn parse_simple_selector(parser: &mut Parser) -> SimpleSelector {
    let mut selector = SimpleSelector {
        tag_name: None,
        id: None,
        class: Vec::new(),
    };
    while !parser.eof() {
        match parser.next_char() {
            '#' => {
                parser.consume_char();
                selector.id = Some(parser.parse_identifier());
            }
            '.' => {
                parser.consume_char();
                selector.class.push(parser.parse_identifier());
            }
            '*' => {
                parser.consume_char();
            }
            c if valid_identifier_char(c) => {
                selector.tag_name = Some(parser.parse_identifier());
            }
            _ => break, // mainly `,`
        }
    }
    return selector;
}

fn parse_declaration(parser: &mut Parser) -> Declaration {
    let dec = parser.consume_while(|c| c != ';');
    println!("dec {}", dec);

    Declaration {
        name: String::from("width"),
        value: Value::Keyword(String::from("inherit")),
    }
}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => true,
        _ => false,
    }
}
