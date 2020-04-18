mod dom;
mod window;
mod http;
mod parser;

pub const BROWSER_NAME: &str = "ToyBrowser";

fn main() {
    // window::open_browser(BROWSER_NAME);

    let client = http::HttpClient::new();

    let a = client.get("file:///home/deepankar/test.html");
    println!("{}", a);

    let dom_tree = parser::html::parse(a);
    println!("{}", dom_tree);
    
    let stylesheet = parser::css::parse(dom::get_css_text(dom_tree));
    println!("{:?}", stylesheet);

}
