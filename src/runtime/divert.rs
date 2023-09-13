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

#[derive(Debug, Deserialize, Default, PartialEq, Eq, Clone)]
#[serde(from = "DivertData")]
pub struct Divert {
    pub target: Option<TargetType>,
    pub stack_push_type: PushPopType,
    pub pushes_to_stack: bool,
    pub external_args: Option<u32>,
    pub is_external: bool,
    pub is_conditional: bool,
}

impl Divert {
    pub fn new() -> Divert {
        Divert::default()
    }

    pub fn new_function() -> Divert {
        Divert {
            stack_push_type: PushPopType::Function,
            pushes_to_stack: true,
            ..Default::default()
        }
    }

    pub fn new_tunnel() -> Divert {
        Divert {
            stack_push_type: PushPopType::Tunnel,
            pushes_to_stack: true,
            ..Default::default()
        }
    }

    pub fn new_external_function() -> Divert {
        Divert {
            stack_push_type: PushPopType::Function,
            is_external: true,
            ..Default::default()
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
        match divert.divert_type {
            DivertType::Standard { path } => Self {
                target: Some(TargetType::Path(path)),
                is_conditional: divert.conditional,
                ..Self::new()
            },
            DivertType::Variable { target, var: () } => Self {
                target: Some(TargetType::VarName(target)),
                is_conditional: divert.conditional,
                ..Self::new()
            },
            DivertType::Function { path } => Self {
                target: Some(TargetType::Path(path)),
                is_conditional: divert.conditional,
                ..Self::new_function()
            },
            DivertType::Tunnel { path } => Self {
                target: Some(TargetType::Path(path)),
                is_conditional: divert.conditional,
                ..Self::new_tunnel()
            },
            DivertType::ExternalFunction {
                external_func_name,
                external_arguments,
            } => Self {
                target: Some(TargetType::ExternalName(external_func_name)),
                external_args: Some(external_arguments),
                is_conditional: divert.conditional,
                ..Self::new_external_function()
            },
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
