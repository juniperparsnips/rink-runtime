use std::fmt;

use serde::Deserialize;

use crate::path::Path;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "ChoicePointData")]
pub struct ChoicePoint {
    pub has_condition: bool,
    pub has_start_content: bool,
    pub has_choice_only_content: bool,
    pub is_invisible_default: bool,
    pub once_only: bool,
    pub choice_target_path: Path,
}

impl ChoicePoint {
    pub fn new(choice_target_path: Path, flags: u8) -> ChoicePoint {
        ChoicePoint {
            has_condition: flags & 0x1 > 0,
            has_start_content: flags & 0x2 > 0,
            has_choice_only_content: flags & 0x4 > 0,
            is_invisible_default: flags & 0x8 > 0,
            once_only: flags & 0x10 > 0,
            choice_target_path,
        }
    }

    pub fn flags(&self) -> u8 {
        let mut flags: u8 = 0;

        if self.has_condition {
            flags |= 0x1;
        }

        if self.has_start_content {
            flags |= 0x2;
        }

        if self.has_choice_only_content {
            flags |= 0x4;
        }

        if self.is_invisible_default {
            flags |= 0x8;
        }

        if self.once_only {
            flags |= 0x10;
        }

        flags
    }
}

impl fmt::Display for ChoicePoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO
        write!(f, "")
    }
}

#[derive(Debug, Deserialize)]
struct ChoicePointData {
    #[serde(rename = "*")]
    choice_target_path: Path,
    #[serde(rename = "flg")]
    flags: u8,
}

impl From<ChoicePointData> for ChoicePoint {
    fn from(
        ChoicePointData {
            choice_target_path,
            flags,
        }: ChoicePointData,
    ) -> Self {
        ChoicePoint::new(choice_target_path, flags)
    }
}
