/// CSS parser
///
/// Example CSS:
///
/// h1, h2, h3 { margin: auto; color: #cc0000; }
/// div.note { margin-bottom: 20px; padding: 10px; }
/// #answer { display: none; }
///
/// Each rule has selectors and declarations applied to it

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
    In,
    Per,
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub fn parse(source: String) -> Stylesheet {
    Stylesheet {
        rules: Vec::new(),
    }
}
