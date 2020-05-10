/// CSS parser
///
/// Example CSS:
///
/// h1, h2, h3 { margin: auto; color: #cc0000; }
/// div.note { margin-bottom: 20px; padding: 10px; }
/// #answer { display: none; }
///
/// Each rule has selectors and declarations applied to it
use super::colors;
use super::length;
use super::Parser;

pub const COLOR_PROPERTIES: [&str; 8] = [
    "background-color",
    "border-color",
    "border-top-color",
    "border-right-color",
    "border-bottom-color",
    "border-left-color",
    "color",
    "outline-color",
];

/// Decides the order in which to apply css properties
///
/// For example, id takes preference over class
pub type Specificity = (usize, usize, usize);

#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,       // h1, h2, h3
    pub declarations: Vec<Declaration>, // { margin: auto; color: #cc0000; }
}

#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector),
}

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

#[derive(Debug)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub enum Value {
    Keyword(String),
    Length(LengthValue, Unit),
    ColorValue(Color),
    // Number(f32),
}

#[derive(Debug, Clone)]
pub enum LengthValue {
    Single(i32),
    All(i32, i32, i32, i32),
}

#[derive(Debug, Clone)]
pub enum Unit {
    Px,
    Em,
}

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
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
                selector.id = Some(parse_identifier(parser));
            }
            '.' => {
                parser.consume_char();
                selector.class.push(parse_identifier(parser));
            }
            '*' => {
                parser.consume_char();
            }
            c if valid_identifier_char(c) => {
                selector.tag_name = Some(parse_identifier(parser));
            }
            _ => break, // mainly `,`
        }
    }
    return selector;
}

fn parse_declaration(parser: &mut Parser) -> Declaration {
    parser.skip_whitespace();

    let prop_name = parse_identifier(parser);
    parser.skip_whitespace();
    assert_eq!(':', parser.consume_char());

    parser.skip_whitespace();
    let prop_value = parse_property_value(
        &prop_name,
        parser
            .consume_while(|c| c != ';')
            .trim()
            .to_ascii_lowercase(),
    );
    assert_eq!(';', parser.consume_char());

    Declaration {
        name: prop_name,
        value: prop_value,
    }
}

fn parse_property_value(property_name: &String, value_string: String) -> Value {
    if COLOR_PROPERTIES.contains(&property_name.as_ref()) {
        let maybe_color = colors::parse_color(value_string.as_ref());
        match maybe_color {
            Ok(c) => return Value::ColorValue(c),
            _ => {} // when color value is inherit, etc.
        }
    }
    match length::parse_length(value_string.as_ref()) {
        Some((le, unit)) => return Value::Length(le, unit),
        None => {}
    }

    Value::Keyword(value_string)
}

fn parse_identifier(parser: &mut Parser) -> String {
    parser.consume_while(valid_identifier_char)
}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => true,
        _ => false,
    }
}
