use std::str::FromStr;

/// Custom error for strings that cannot be parsed into chords.
#[derive(Debug)]
pub struct ParseThemeError {
    name: String,
}

#[derive(Clone)]
pub enum Theme {
    Light,
    Dark,
}

impl FromStr for Theme {
    type Err = ParseThemeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Theme::*;

        let name = s.to_string();

        if s == "L" {
            Ok(Light)
        } else if s == "D" {
            Ok(Dark)
        } else {
            Err(ParseThemeError { name })
        }
    }
}