use std::fmt;

use serde::Deserialize;

/// Additional structure to hold RuntimeObject
/// 
/// With the current implementation, the parser expect a map. If a container is
/// provided (= a list), the parser will panic. Therefore, we create this
/// additional holder, which is a map, which can be deserialized with its
/// content. We can use it to test the parsing of all the other objects without
/// having to implement all the header of a full story.
#[derive(Debug, Deserialize)]
pub struct TestContainer {
    text: String,
}

// WIP: create the structure and allow for empty fields


impl TestHolder {
    pub fn new(text: String) -> Tag {
        Tag { text: text }
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.text)
    }
}
