use thiserror::Error;

use self::depth::HeadingDepth;
use crate::ast::ParagraphChild;

pub mod depth;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Heading {
    pub children: Vec<ParagraphChild>,
    pub position: crate::unist::Position,
    pub depth: HeadingDepth,
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("todo")]
    Child(#[from] crate::ast::paragraph::child::ConvertError),
    #[error("todo")]
    InvalidDepth(#[from] self::depth::ConvertError),
    #[error("todo")]
    NoPosition,
}

impl TryFrom<markdown::mdast::Heading> for Heading {
    type Error = ConvertError;

    fn try_from(value: markdown::mdast::Heading) -> Result<Self, Self::Error> {
        Ok(Self {
            children: {
                value
                    .children
                    .into_iter()
                    .map(ParagraphChild::try_from)
                    .collect::<Result<Vec<_>, _>>()?
            },
            position: value.position.ok_or(ConvertError::NoPosition)?.into(),
            depth: value.depth.try_into()?,
        })
    }
}
