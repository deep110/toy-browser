/// CSS parser
///
/// Example CSS:
///
/// h1, h2, h3 { margin: auto; color: #cc0000; }
/// div.note { margin-bottom: 20px; padding: 10px; }
/// #answer { display: none; }
///
/// Each rule has selectors and declarations applied to it

struct Stylesheet {
    rules: Vec<Rule>,
}

struct Rule {
    selectors: Vec<Selector>,       // h1, h2, h3
    declarations: Vec<Declaration>, // { margin: auto; color: #cc0000; }
}

enum Selector {
    Simple(SimpleSelector),
}

struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

struct Declaration {
    name: String,
    value: Value,
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

enum Unit {
    Px,
    In,
    Per,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
