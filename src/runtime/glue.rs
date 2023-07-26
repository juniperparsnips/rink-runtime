use std::fmt;

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub enum Glue {
    #[serde(rename = "<>")]
    Bidirectional,
    #[serde(rename = "G<")]
    Left,
    #[serde(rename = "G>")]
    Right,
}

impl fmt::Display for Glue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Glue::Bidirectional => write!(f, "<>"),
            Glue::Left => write!(f, "G<"),
            Glue::Right => write!(f, "G>"),
        }
    }
}
