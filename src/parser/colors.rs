use super::css::Color;
use crate::{errors, Result};

pub fn parse_color(color_string: String) -> Color {
    Color::new()
}

fn parse_hex(hex_string: String) -> Result<Color> {
    let len = hex_string.len();
    if len >= 7 {
        let mut color = Color::new();
        color.r = u8::from_str_radix(&hex_string[1..3], 16)?;
        color.g = u8::from_str_radix(&hex_string[3..5], 16)?;
        color.b = u8::from_str_radix(&hex_string[5..7], 16)?;

        if len == 9 {
            // also add alpha
            color.a = u8::from_str_radix(&hex_string[7..9], 16)?;
        }

        return Ok(color);
    }
    return errors::parse_error("wrong color hex");
}

#[test]
fn test_parse_hex() {
    assert_eq!(
        parse_hex(String::from("#ffffff")).unwrap(),
        Color::from(255, 255, 255, 255)
    );
    assert_eq!(
        parse_hex(String::from("#7fffd4")).unwrap(),
        Color::from(127, 255, 212, 255)
    );
    assert_eq!(
        parse_hex(String::from("#daa520")).unwrap(),
        Color::from(218, 165, 32, 255)
    );
    assert_eq!(
        parse_hex(String::from("#40e0d0a0")).unwrap(),
        Color::from(64, 224, 208, 160)
    );
}
