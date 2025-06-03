use thiserror::Error;

use crate::ast::ParagraphChild;
use crate::unist::Position;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Emphasis {
    pub children: Vec<ParagraphChild>,
    pub position: Position,
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("todo")]
    Child(#[from] Box<crate::ast::paragraph::child::ConvertError>),
    #[error("todo")]
    NoPosition,
}

impl TryFrom<markdown::mdast::Emphasis> for Emphasis {
    type Error = ConvertError;

    fn try_from(value: markdown::mdast::Emphasis) -> Result<Self, Self::Error> {
        Ok(Self {
            children: {
                value
                    .children
                    .into_iter()
                    .map(ParagraphChild::try_from)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(Box::new)?
            },
            position: value.position.ok_or(ConvertError::NoPosition)?.into(),
        })
    }
}
