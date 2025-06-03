use thiserror::Error;

use self::item::ListItem;
use crate::unist::Position;

pub mod item;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct List {
    pub children: Vec<ListItem>,
    pub position: Position,
    pub ordered: bool,
    pub spread: bool,
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("todo")]
    Item(#[from] self::item::ConvertError),
    #[error("todo")]
    NotItem(markdown::mdast::Node),
    #[error("todo")]
    NoPosition,
}

impl TryFrom<markdown::mdast::List> for List {
    type Error = ConvertError;

    fn try_from(value: markdown::mdast::List) -> Result<Self, Self::Error> {
        Ok(Self {
            children: {
                value
                    .children
                    .into_iter()
                    .map(|node| -> Result<ListItem, Self::Error> {
                        if let markdown::mdast::Node::ListItem(li) = node {
                            Ok(ListItem::try_from(li)?)
                        } else {
                            Err(ConvertError::NotItem(node))
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?
            },
            position: value.position.ok_or(ConvertError::NoPosition)?.into(),
            ordered: value.ordered,
            spread: value.spread,
        })
    }
}
