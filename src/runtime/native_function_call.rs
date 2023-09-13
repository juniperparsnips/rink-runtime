use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum NativeFunctionCall {
    #[serde(rename = "+")]
    Plus,
    #[serde(rename = "-")]
    Minus,
    #[serde(rename = "/")]
    Divide,
    #[serde(rename = "*")]
    Multiply,
    #[serde(rename = "%")]
    Modulo,
    #[serde(rename = "_")]
    UnaryMinus,
    #[serde(rename = "==")]
    Eq,
    #[serde(rename = ">")]
    GT,
    #[serde(rename = "<")]
    LT,
    #[serde(rename = ">=")]
    GEq,
    #[serde(rename = "<=")]
    LEq,
    #[serde(rename = "!=")]
    NEq,
    #[serde(rename = "!")]
    UnaryNot,
    #[serde(rename = "&&")]
    And,
    #[serde(rename = "||")]
    Or,
    #[serde(rename = "MIN")]
    Min,
    #[serde(rename = "MAX")]
    Max,
}
