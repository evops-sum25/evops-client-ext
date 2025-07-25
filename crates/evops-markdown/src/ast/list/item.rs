use thiserror::Error;

use crate::ast::MarkdownRootChild;
use crate::unist::MarkdownPosition;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct MarkdownListItem {
    pub children: Vec<MarkdownRootChild>,
    pub position: MarkdownPosition,
    pub spread: bool,
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("todo")]
    Child(#[from] Box<crate::ast::root::child::ConvertError>),
    #[error("todo")]
    NoPosition,
}

impl TryFrom<markdown::mdast::ListItem> for MarkdownListItem {
    type Error = ConvertError;

    fn try_from(value: markdown::mdast::ListItem) -> Result<Self, Self::Error> {
        Ok(Self {
            children: {
                value
                    .children
                    .into_iter()
                    .map(MarkdownRootChild::try_from)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(Box::new)?
            },
            position: value.position.ok_or(ConvertError::NoPosition)?.into(),
            // todo: figure out its meaning
            spread: value.spread,
        })
    }
}
