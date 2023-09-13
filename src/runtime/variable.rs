use std::fmt;

use serde::Deserialize;

use crate::path::Path;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "VariableAssignmentData")]
pub struct VariableAssignment {
    pub name: String,
    pub is_new_declaration: bool,
    pub is_global: bool,
}

impl fmt::Display for VariableAssignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VarAssign to {}", self.name)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct VariableReference {
    #[serde(rename = "VAR?")]
    pub name: String,
}

impl fmt::Display for VariableReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var({})", self.name)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ReadCount {
    #[serde(rename = "CNT?")]
    pub target: Path,
}

impl fmt::Display for ReadCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "read_count({})", self.target.to_string())
    }
}

#[derive(Debug, Deserialize)]
struct VariableAssignmentData {
    #[serde(flatten)]
    assignment_type: VariableAssignmentType,
    #[serde(rename = "re", default)]
    old_declaration: bool,
}

#[derive(Debug, Deserialize)]
enum VariableAssignmentType {
    #[serde(rename = "VAR=")]
    Global(String),
    #[serde(rename = "temp=")]
    Temporary(String),
}

impl From<VariableAssignmentData> for VariableAssignment {
    fn from(
        VariableAssignmentData {
            assignment_type,
            old_declaration,
        }: VariableAssignmentData,
    ) -> Self {
        let is_new_declaration = !old_declaration;
        match assignment_type {
            VariableAssignmentType::Global(name) => VariableAssignment {
                name,
                is_new_declaration,
                is_global: true,
            },
            VariableAssignmentType::Temporary(name) => VariableAssignment {
                name,
                is_new_declaration,
                is_global: false,
            },
        }
    }
}
