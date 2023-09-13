use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Tag {
    #[serde(rename = "#")]
    pub text: String,
}


impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.text)
    }
}
