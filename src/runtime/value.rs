use std::fmt;

use serde::{de::Error, Deserialize, Deserializer};

use crate::path::Path;

fn from_prefixed_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s == "\n" {
        return Ok("\n".to_owned());
    }
    Ok(s.strip_prefix("^")
        .ok_or(D::Error::custom("String does not begin with '^'"))?
        .to_owned())
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum Value {
    Int(i32),
    Float(f32),
    //List,
    #[serde(deserialize_with = "from_prefixed_string")]
    String(String),
    DivertTarget {
        #[serde(rename = "^->")]
        target_path: Path,
    },
    VariablePointer {
        #[serde(rename = "^var")]
        name: String,
        #[serde(rename = "ci")]
        context_index: i32,
    },
}

impl Value {
    pub fn as_int(&self) -> Option<i32> {
        match self {
            &Value::Int(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            &Value::Float(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::DivertTarget { target_path } => {
                write!(f, "DivertTarget({})", target_path)
            }
            Value::VariablePointer {
                name,
                context_index,
            } => write!(f, "VarPtr({})", name),
        }
    }
}
