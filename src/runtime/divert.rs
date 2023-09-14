use serde::{de::Error, Deserialize, Deserializer};

use crate::path::Path;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Default)]
pub enum PushPopType {
    Tunnel,
    Function,
    #[default]
    None,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum TargetType {
    VarName(String),
    ExternalName(String),
    Path(Path),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "DivertData")]
pub struct Divert {
    pub target: TargetType,
    pub stack_push_type: PushPopType,
    pub pushes_to_stack: bool,
    pub external_args: Option<u32>,
    pub is_conditional: bool,
}

impl Divert {
    pub fn new(target: TargetType) -> Divert {
        Divert {
            target,
            stack_push_type: PushPopType::None,
            pushes_to_stack: false,
            external_args: None,
            is_conditional: false,
        }
    }

    pub fn new_divert(target: TargetType) -> Divert {
        Divert::new(target)
    }

    pub fn new_function(target: TargetType) -> Divert {
        Divert {
            pushes_to_stack: true,
            stack_push_type: PushPopType::Function,
            ..Divert::new(target)
        }
    }

    pub fn new_tunnel(target: TargetType) -> Divert {
        Divert {
            pushes_to_stack: true,
            stack_push_type: PushPopType::Tunnel,
            ..Divert::new(target)
        }
    }

    pub fn new_external_function(target: TargetType, external_args: u32) -> Divert {
        Divert {
            stack_push_type: PushPopType::Function,
            external_args: Some(external_args),
            ..Divert::new(target)
        }
    }
}

#[derive(Debug, Deserialize)]
struct DivertData {
    #[serde(flatten)]
    divert_type: DivertType,
    #[serde(rename = "c", default)]
    conditional: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
enum DivertType {
    Standard {
        #[serde(rename = "->")]
        path: Path,
    },
    Variable {
        #[serde(rename = "->")]
        target: String,
        #[serde(deserialize_with = "true_bool")]
        var: (),
    },
    Function {
        #[serde(rename = "f()")]
        path: Path,
    },
    Tunnel {
        #[serde(rename = "->t->")]
        path: Path,
    },
    ExternalFunction {
        #[serde(rename = "x()")]
        external_func_name: String,
        #[serde(rename = "exArgs", default)]
        external_arguments: u32,
    },
}

impl From<DivertData> for Divert {
    fn from(divert: DivertData) -> Self {
        let new_divert = match divert.divert_type {
            DivertType::Standard { path } => Self::new_divert(TargetType::Path(path)),
            DivertType::Variable { target, var: () } => {
                Self::new_divert(TargetType::VarName(target))
            }
            DivertType::Function { path } => Self::new_function(TargetType::Path(path)),
            DivertType::Tunnel { path } => Self::new_tunnel(TargetType::Path(path)),
            DivertType::ExternalFunction {
                external_func_name,
                external_arguments,
            } => Self::new_external_function(
                TargetType::ExternalName(external_func_name),
                external_arguments,
            ),
        };
        Self {
            is_conditional: divert.conditional,
            ..new_divert
        }
    }
}

fn true_bool<'de, D>(deserializer: D) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
    D::Error: Error,
{
    let b: bool = Deserialize::deserialize(deserializer)?;
    if b {
        Ok(())
    } else {
        Err(D::Error::custom("Failed, bool literal must be set to true"))
    }
}
