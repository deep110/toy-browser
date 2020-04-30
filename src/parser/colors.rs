use super::css::Color;
use crate::{errors, Result};

pub fn parse_color(color_string: String) -> Color {
    Color::new()
}

fn parse_hex(hex_string: String) -> Result<Color> {
    let len = hex_string.len();
    if len == 4 {
        let mut color = Color::new();
        color.r = u8::from_str_radix(
            format!("{}{}", &hex_string[1..2], &hex_string[1..2]).as_str(),
            16,
        )?;
        color.g = u8::from_str_radix(
            format!("{}{}", &hex_string[2..3], &hex_string[2..3]).as_str(),
            16,
        )?;
        color.b = u8::from_str_radix(
            format!("{}{}", &hex_string[3..], &hex_string[3..]).as_str(),
            16,
        )?;
        return Ok(color);
    }

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
    assert_eq!(
        parse_hex(String::from("#adf")).unwrap(),
        Color::from(170, 221, 255, 255)
    );
}

fn parse_color_name(color_name: String) -> Result<Color> {
    match color_name.as_str() {
        "black" => Ok(Color::from(0, 0, 0, 255)),
        "silver" => Ok(Color::from(192, 192, 192, 255)),
        "gray" => Ok(Color::from(128, 128, 128, 255)),
        "white" => Ok(Color::from(255, 255, 255, 255)),
        "maroon" => Ok(Color::from(128, 0, 0, 255)),
        "red" => Ok(Color::from(255, 0, 0, 255)),
        "purple" => Ok(Color::from(128, 0, 128, 255)),
        "fuchsia" => Ok(Color::from(255, 0, 255, 255)),
        "green" => Ok(Color::from(0, 128, 0, 255)),
        "lime" => Ok(Color::from(0, 255, 0, 255)),
        "olive" => Ok(Color::from(128, 128, 0, 255)),
        "yellow" => Ok(Color::from(255, 255, 0, 255)),
        "navy" => Ok(Color::from(0, 0, 128, 255)),
        "blue" => Ok(Color::from(0, 0, 255, 255)),
        "teal" => Ok(Color::from(0, 128, 128, 255)),
        "aqua" => Ok(Color::from(0, 255, 255, 255)),
        "orange" => Ok(Color::from(255, 165, 0, 255)),
        "aliceblue" => Ok(Color::from(240, 248, 255, 255)),
        "antiquewhite" => Ok(Color::from(250, 235, 215, 255)),
        "aquamarine" => Ok(Color::from(127, 255, 212, 255)),
        "azure" => Ok(Color::from(240, 255, 255, 255)),
        "beige" => Ok(Color::from(245, 245, 220, 255)),
        "bisque" => Ok(Color::from(255, 228, 196, 255)),
        "blanchedalmond" => Ok(Color::from(255, 235, 205, 255)),
        "blueviolet" => Ok(Color::from(138, 43, 226, 255)),
        "brown" => Ok(Color::from(165, 42, 42, 255)),
        "burlywood" => Ok(Color::from(222, 184, 135, 255)),
        "cadetblue" => Ok(Color::from(95, 158, 160, 255)),
        "chartreuse" => Ok(Color::from(127, 255, 0, 255)),
        "chocolate" => Ok(Color::from(210, 105, 30, 255)),
        "coral" => Ok(Color::from(255, 127, 80, 255)),
        "cornflowerblue" => Ok(Color::from(100, 149, 237, 255)),
        "cornsilk" => Ok(Color::from(255, 248, 220, 255)),
        "crimson" => Ok(Color::from(220, 20, 60, 255)),
        "cyan" => Ok(Color::from(0, 255, 255, 255)),
        "darkblue" => Ok(Color::from(0, 0, 139, 255)),
        "darkcyan" => Ok(Color::from(0, 139, 139, 255)),
        "darkgoldenrod" => Ok(Color::from(184, 134, 11, 255)),
        "darkgray" => Ok(Color::from(169, 169, 169, 255)),
        "darkgreen" => Ok(Color::from(0, 100, 0, 255)),
        "darkgrey" => Ok(Color::from(169, 169, 169, 255)),
        "darkkhaki" => Ok(Color::from(189, 183, 107, 255)),
        "darkmagenta" => Ok(Color::from(139, 0, 139, 255)),
        "darkolivegreen" => Ok(Color::from(85, 107, 47, 255)),
        "darkorange" => Ok(Color::from(255, 140, 0, 255)),
        "darkorchid" => Ok(Color::from(153, 50, 204, 255)),
        "darkred" => Ok(Color::from(139, 0, 0, 255)),
        "darksalmon" => Ok(Color::from(233, 150, 122, 255)),
        "darkseagreen" => Ok(Color::from(143, 188, 143, 255)),
        "darkslateblue" => Ok(Color::from(72, 61, 139, 255)),
        "darkslategray" => Ok(Color::from(47, 79, 79, 255)),
        "darkslategrey" => Ok(Color::from(47, 79, 79, 255)),
        "darkturquoise" => Ok(Color::from(0, 206, 209, 255)),
        "darkviolet" => Ok(Color::from(148, 0, 211, 255)),
        "deeppink" => Ok(Color::from(255, 20, 147, 255)),
        "deepskyblue" => Ok(Color::from(0, 191, 255, 255)),
        "dimgray" => Ok(Color::from(105, 105, 105, 255)),
        "dimgrey" => Ok(Color::from(105, 105, 105, 255)),
        "dodgerblue" => Ok(Color::from(30, 144, 255, 255)),
        "firebrick" => Ok(Color::from(178, 34, 34, 255)),
        "floralwhite" => Ok(Color::from(255, 250, 240, 255)),
        "forestgreen" => Ok(Color::from(34, 139, 34, 255)),
        "gainsboro" => Ok(Color::from(220, 220, 220, 255)),
        "ghostwhite" => Ok(Color::from(248, 248, 255, 255)),
        "gold" => Ok(Color::from(255, 215, 0, 255)),
        "goldenrod" => Ok(Color::from(218, 165, 32, 255)),
        "greenyellow" => Ok(Color::from(173, 255, 47, 255)),
        "grey" => Ok(Color::from(128, 128, 128, 255)),
        "honeydew" => Ok(Color::from(240, 255, 240, 255)),
        "hotpink" => Ok(Color::from(255, 105, 180, 255)),
        "indianred" => Ok(Color::from(205, 92, 92, 255)),
        "indigo" => Ok(Color::from(75, 0, 130, 255)),
        "ivory" => Ok(Color::from(255, 255, 240, 255)),
        "khaki" => Ok(Color::from(240, 230, 140, 255)),
        "lavender" => Ok(Color::from(230, 230, 250, 255)),
        "lavenderblush" => Ok(Color::from(255, 240, 245, 255)),
        "lawngreen" => Ok(Color::from(124, 252, 0, 255)),
        "lemonchiffon" => Ok(Color::from(255, 250, 205, 255)),
        "lightblue" => Ok(Color::from(173, 216, 230, 255)),
        "lightcoral" => Ok(Color::from(240, 128, 128, 255)),
        "lightcyan" => Ok(Color::from(224, 255, 255, 255)),
        "lightgoldenrodyellow" => Ok(Color::from(250, 250, 210, 255)),
        "lightgray" => Ok(Color::from(211, 211, 211, 255)),
        "lightgreen" => Ok(Color::from(144, 238, 144, 255)),
        "lightgrey" => Ok(Color::from(211, 211, 211, 255)),
        "lightpink" => Ok(Color::from(255, 182, 193, 255)),
        "lightsalmon" => Ok(Color::from(255, 160, 122, 255)),
        "lightseagreen" => Ok(Color::from(32, 178, 170, 255)),
        "lightskyblue" => Ok(Color::from(135, 206, 250, 255)),
        "lightslategray" => Ok(Color::from(119, 136, 153, 255)),
        "lightslategrey" => Ok(Color::from(119, 136, 153, 255)),
        "lightsteelblue" => Ok(Color::from(176, 196, 222, 255)),
        "lightyellow" => Ok(Color::from(255, 255, 224, 255)),
        "limegreen" => Ok(Color::from(50, 205, 50, 255)),
        "linen" => Ok(Color::from(250, 240, 230, 255)),
        "magenta" => Ok(Color::from(255, 0, 255, 255)),
        "mediumaquamarine" => Ok(Color::from(102, 205, 170, 255)),
        "mediumblue" => Ok(Color::from(0, 0, 205, 255)),
        "mediumorchid" => Ok(Color::from(186, 85, 211, 255)),
        "mediumpurple" => Ok(Color::from(147, 112, 219, 255)),
        "mediumseagreen" => Ok(Color::from(60, 179, 113, 255)),
        "mediumslateblue" => Ok(Color::from(123, 104, 238, 255)),
        "mediumspringgreen" => Ok(Color::from(0, 250, 154, 255)),
        "mediumturquoise" => Ok(Color::from(72, 209, 204, 255)),
        "mediumvioletred" => Ok(Color::from(199, 21, 133, 255)),
        "midnightblue" => Ok(Color::from(25, 25, 112, 255)),
        "mintcream" => Ok(Color::from(245, 255, 250, 255)),
        "mistyrose" => Ok(Color::from(255, 228, 225, 255)),
        "moccasin" => Ok(Color::from(255, 228, 181, 255)),
        "navajowhite" => Ok(Color::from(255, 222, 173, 255)),
        "oldlace" => Ok(Color::from(253, 245, 230, 255)),
        "olivedrab" => Ok(Color::from(107, 142, 35, 255)),
        "orangered" => Ok(Color::from(255, 69, 0, 255)),
        "orchid" => Ok(Color::from(218, 112, 214, 255)),
        "palegoldenrod" => Ok(Color::from(238, 232, 170, 255)),
        "palegreen" => Ok(Color::from(152, 251, 152, 255)),
        "paleturquoise" => Ok(Color::from(175, 238, 238, 255)),
        "palevioletred" => Ok(Color::from(219, 112, 147, 255)),
        "papayawhip" => Ok(Color::from(255, 239, 213, 255)),
        "peachpuff" => Ok(Color::from(255, 218, 185, 255)),
        "peru" => Ok(Color::from(205, 133, 63, 255)),
        "pink" => Ok(Color::from(255, 192, 203, 255)),
        "plum" => Ok(Color::from(221, 160, 221, 255)),
        "powderblue" => Ok(Color::from(176, 224, 230, 255)),
        "rosybrown" => Ok(Color::from(188, 143, 143, 255)),
        "royalblue" => Ok(Color::from(65, 105, 225, 255)),
        "saddlebrown" => Ok(Color::from(139, 69, 19, 255)),
        "salmon" => Ok(Color::from(250, 128, 114, 255)),
        "sandybrown" => Ok(Color::from(244, 164, 96, 255)),
        "seagreen" => Ok(Color::from(46, 139, 87, 255)),
        "seashell" => Ok(Color::from(255, 245, 238, 255)),
        "sienna" => Ok(Color::from(160, 82, 45, 255)),
        "skyblue" => Ok(Color::from(135, 206, 235, 255)),
        "slateblue" => Ok(Color::from(106, 90, 205, 255)),
        "slategray" => Ok(Color::from(112, 128, 144, 255)),
        "slategrey" => Ok(Color::from(112, 128, 144, 255)),
        "snow" => Ok(Color::from(255, 250, 250, 255)),
        "springgreen" => Ok(Color::from(0, 255, 127, 255)),
        "steelblue" => Ok(Color::from(70, 130, 180, 255)),
        "tan" => Ok(Color::from(210, 180, 140, 255)),
        "thistle" => Ok(Color::from(216, 191, 216, 255)),
        "tomato" => Ok(Color::from(255, 99, 71, 255)),
        "turquoise" => Ok(Color::from(64, 224, 208, 255)),
        "violet" => Ok(Color::from(238, 130, 238, 255)),
        "wheat" => Ok(Color::from(245, 222, 179, 255)),
        "whitesmoke" => Ok(Color::from(245, 245, 245, 255)),
        "yellowgreen" => Ok(Color::from(154, 205, 50, 255)),
        "rebeccapurple" => Ok(Color::from(102, 51, 153, 255)),
        _ => errors::parse_error("no matching color found"),
    }
}

#[test]
fn test_parse_color_name() {
    assert_eq!(
        parse_color_name(String::from("blue")).unwrap(),
        Color::from(0, 0, 255, 255)
    );
    assert_eq!(
        parse_color_name(String::from("teal")).unwrap(),
        Color::from(0, 128, 128, 255)
    );
}