use std::str::FromStr;
use crate::svg_draw::settings::Instrument::Guitar;

/// Custom error for strings that cannot be parsed into chords.
#[derive(Debug)]
pub struct ParseThemeError {
    name: String,
}

#[derive(Clone)]
pub enum Theme {
    Light(Instrument),
    Dark(Instrument),
}

#[derive(Clone)]
pub enum Instrument {
    Guitar,
    Piano,
}

impl FromStr for Theme {
    type Err = ParseThemeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Theme::*;

        let name = s.to_string();

        match s {
            "L" => Ok(Light(Guitar)),
            "D" => Ok(Dark(Guitar)),
            _ => Err(ParseThemeError { name })
        }
    }
}