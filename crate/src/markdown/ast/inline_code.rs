use thiserror::Error;

use crate::markdown::unist::Position;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct InlineCode {
    pub value: String,
    pub position: Position,
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("todo")]
    NoPosition,
}

impl TryFrom<markdown::mdast::InlineCode> for InlineCode {
    type Error = ConvertError;

    fn try_from(value: markdown::mdast::InlineCode) -> Result<Self, Self::Error> {
        Ok(Self {
            value: value.value,
            position: value.position.ok_or(ConvertError::NoPosition)?.into(),
        })
    }
}
