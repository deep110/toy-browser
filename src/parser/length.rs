use super::css::{LengthValue, Unit};

use regex::Regex;

lazy_static! {
    static ref LENGTH_RE: Regex = Regex::new(r"((\d+)(px|em))+").unwrap();
}

pub fn parse_length(text: &str) -> Option<(LengthValue, Unit)> {
    let res: Vec<_> = LENGTH_RE
        .captures_iter(text)
        .map(|cap| match (cap.get(2), cap.get(3)) {
            (Some(num), Some(unit)) => Some((num.as_str(), unit.as_str())),
            _ => None,
        })
        .filter(|c| !c.is_none())
        .collect();

    match res.len() {
        1 => {
            let u = res[0].unwrap();
            return Some((LengthValue::Single(parse_num(u.0)), parse_unit(u.1)));
        }
        2 => {
            let u = res[0].unwrap();
            let v = res[1].unwrap();
            return Some((
                LengthValue::All(
                    parse_num(u.0),
                    parse_num(v.0),
                    parse_num(u.0),
                    parse_num(v.0),
                ),
                parse_unit(u.1),
            ));
        }
        4 => {
            let u = res[0].unwrap();
            let v = res[1].unwrap();
            let w = res[2].unwrap();
            let z = res[3].unwrap();
            return Some((
                LengthValue::All(
                    parse_num(u.0),
                    parse_num(v.0),
                    parse_num(w.0),
                    parse_num(z.0),
                ),
                parse_unit(u.1),
            ));
        }
        _ => return None,
    }
}

fn parse_unit(unit_text: &str) -> Unit {
    match unit_text {
        "em" => Unit::Em,
        _ => Unit::Px,
    }
}

fn parse_num(num_text: &str) -> i32 {
    num_text.parse::<i32>().unwrap_or(0)
}
