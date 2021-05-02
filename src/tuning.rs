use ukebox::{Semitones};
use std::fmt;
use std::str::FromStr;
use crate::interval::Interval;

// Custom error for strings that cannot be parsed into notes.
#[derive(Debug)]
pub struct ParseTuningError {
    pub name: String,
}

impl fmt::Display for ParseTuningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse tuning name \"{}\"", self.name)
    }
}

/// Tuning
#[derive(Debug, Clone, Copy)]
pub enum Tuning {
    E,
}

impl Tuning {
    /// Normal tuning of a guitar
    pub fn get_semitones(self) -> Semitones {
        match self {
            Self::E => 0,
        }
    }

    /// Interval tuning
    /// See more detail on project ukebox: https://github.com/noeddl/ukebox/blob/master/src/tuning.rs
    /// I have no idea of other tuning with a guitar
    pub fn get_interval(self) -> Interval {
        match self {
            Self::E => Interval::PerfectUnison,
        }
    }
}

impl FromStr for Tuning {
    type Err = ParseTuningError;

    /// Tuning from String
    /// Default tuning "E" or nothing
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.to_string();

        let tuning = match s {
            "E" => Tuning::E,
            _ => return Err(ParseTuningError { name }),
        };

        Ok(tuning)
    }
}