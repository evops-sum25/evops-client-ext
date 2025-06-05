use thiserror::Error;

use self::child::LinkChild;
use crate::markdown::unist::Position;

pub mod child;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Link {
    pub children: Vec<LinkChild>,
    pub position: Position,
    pub url: String,
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("todo")]
    Child(#[from] crate::markdown::ast::link::child::ConvertError),
    #[error("todo")]
    NoPosition,
}

impl TryFrom<markdown::mdast::Link> for Link {
    type Error = ConvertError;

    fn try_from(value: markdown::mdast::Link) -> Result<Self, Self::Error> {
        // todo: figure out what value.title is

        Ok(Self {
            children: {
                value
                    .children
                    .into_iter()
                    .map(LinkChild::try_from)
                    .collect::<Result<Vec<_>, _>>()?
            },
            position: value.position.ok_or(ConvertError::NoPosition)?.into(),
            url: value.url,
        })
    }
}
