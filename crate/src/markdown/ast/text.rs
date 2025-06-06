use thiserror::Error;

use crate::markdown::unist::Position;

#[derive(Debug)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Text {
    pub value: String,
    pub position: Position,
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("todo")]
    NoPosition,
}

impl TryFrom<markdown::mdast::Text> for Text {
    type Error = ConvertError;

    fn try_from(value: markdown::mdast::Text) -> Result<Self, Self::Error> {
        Ok(Self {
            value: value.value,
            position: value.position.ok_or(ConvertError::NoPosition)?.into(),
        })
    }
}
