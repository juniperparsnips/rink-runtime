use std::{fmt, rc::Rc};

use serde::{de::Error, Deserialize, Deserializer};

use crate::runtime::{
    choice_point::ChoicePoint,
    container::Container,
    control_command::ControlCommand,
    divert::Divert,
    native_function_call::NativeFunctionCall,
    tag::Tag,
    value::Value,
    variable::{ReadCount, VariableAssignment, VariableReference},
};

pub mod story;
pub mod choice_point;
pub mod container;
pub mod control_command;
pub mod divert;
pub mod native_function_call;
pub mod tag;
pub mod value;
pub mod variable;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum RuntimeObject {
    Choice(ChoicePoint),
    Container(Rc<Container>),
    ControlCommand(ControlCommand),
    Divert(Divert),
    NativeFunctionCall(NativeFunctionCall),
    Tag(Tag),
    Value(Value),
    VariableAssignment(VariableAssignment),
    VariableReference(VariableReference),
    ReadCount(ReadCount),
    #[serde(deserialize_with = "glue")]
    Glue,
    #[serde(deserialize_with = "void")]
    Void,
}

impl fmt::Display for RuntimeObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeObject::ControlCommand(control_command) => {
                write!(f, "{}", control_command)
            }
            _ => write!(f, "TODO"),
        }
    }
}

impl RuntimeObject {
    pub fn is_container(&self) -> bool {
        match self {
            RuntimeObject::Container(_) => true,
            _ => false,
        }
    }

    pub fn as_container(&self) -> Option<&Rc<Container>> {
        match self {
            RuntimeObject::Container(container) => Some(container),
            _ => None,
        }
    }

    pub fn as_value(&self) -> Option<&Value> {
        match self {
            RuntimeObject::Value(value) => Some(value),
            _ => None,
        }
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            RuntimeObject::Container(container) => match container.name.as_ref() {
                Some(name) => Some(name),
                _ => None,
            },
            // TODO
            _ => None,
        }
    }
}

fn void<'de, D>(deserializer: D) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
    D::Error: Error,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s == "void" {
        Ok(())
    } else {
        Err(D::Error::custom(
            "Failed to deserialize string literal as void",
        ))
    }
}

fn glue<'de, D>(deserializer: D) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
    D::Error: Error,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s == "<>" {
        Ok(())
    } else {
        Err(D::Error::custom(
            "Failed to deserialize string literal as glue",
        ))
    }
}
