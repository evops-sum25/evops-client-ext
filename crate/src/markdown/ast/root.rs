use thiserror::Error;

use self::child::RootChild;
use crate::markdown::unist::Position;

pub mod child;

#[derive(Debug)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Root {
    pub children: Vec<RootChild>,
    pub position: Position,
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("todo")]
    Child(#[from] self::child::ConvertError),
    #[error("todo")]
    NoPosition,
}

impl TryFrom<markdown::mdast::Root> for Root {
    type Error = ConvertError;

    fn try_from(value: markdown::mdast::Root) -> Result<Self, Self::Error> {
        Ok(Self {
            children: {
                value
                    .children
                    .into_iter()
                    .map(RootChild::try_from)
                    .collect::<Result<_, _>>()?
            },
            position: value.position.ok_or(ConvertError::NoPosition)?.into(),
        })
    }
}
