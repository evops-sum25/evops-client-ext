use thiserror::Error;

use crate::markdown::ast::RootChild;
use crate::markdown::unist::Position;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ListItem {
    pub children: Vec<RootChild>,
    pub position: Position,
    pub spread: bool,
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("todo")]
    Child(#[from] Box<crate::markdown::ast::root::child::ConvertError>),
    #[error("todo")]
    NoPosition,
}

impl TryFrom<markdown::mdast::ListItem> for ListItem {
    type Error = ConvertError;

    fn try_from(value: markdown::mdast::ListItem) -> Result<Self, Self::Error> {
        Ok(Self {
            children: {
                value
                    .children
                    .into_iter()
                    .map(RootChild::try_from)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(Box::new)?
            },
            position: value.position.ok_or(ConvertError::NoPosition)?.into(),
            // todo: figure out its meaning
            spread: value.spread,
        })
    }
}
