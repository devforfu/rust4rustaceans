use std::str::FromStr;

pub enum Unit {
    Grams(i32),
    Kilograms(i32),
    Tonnes(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseUnitError;

impl FromStr for Unit {
    type Err = ParseUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(unit) = parse_unit(s) {
            Ok(unit)
        } else {
            Err(ParseUnitError)
        }
    }
}

fn parse_unit(s: &str) -> Option<Unit> {
    #[derive(PartialEq)]
    enum State {Digit, Unit}

    let mut curr = State::Digit;
    let mut num: i32 = 0; 
    let mut unit = String::new();

    for ch in s.to_lowercase().bytes() {
        match ch {
            b'0'..=b'9' => {
                if curr == State::Unit {
                    return None;
                }
                num += 10*((ch - b'0') as i32)
            },
            b'a'..=b'z' => {
                if curr == State::Digit {
                    curr = State::Unit;
                }
                unit.push(ch as char); 
            },
            _ => (),
        }
    }

    match unit.as_str() {
        "g" => Some(Unit::Grams(num)),
        "kg" => Some(Unit::Kilograms(num)),
        "t" | "tn" => Some(Unit::Tonnes(num)),
        _ => return None, 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unit() {
        matches!(parse_unit("1000kg"), Some(Unit::Kilograms(1000)));
        matches!(parse_unit("123g"), Some(Unit::Grams(123)));
        matches!(parse_unit("500t"), Some(Unit::Tonnes(500)));
        matches!(parse_unit("25tn"), Some(Unit::Tonnes(25)));
        matches!(parse_unit("000tnt"), None);
        matches!(parse_unit("200"), None);
        matches!(parse_unit("kg"), None);
    }

    #[test]
    fn test_from_str() {
        matches!(Unit::from_str("10g"), Ok(Unit::Grams(10)));
        matches!(Unit::from_str("28m"), Err(ParseUnitError));
    }
}
