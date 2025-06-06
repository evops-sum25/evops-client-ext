use thiserror::Error;

use crate::markdown::ast::{
    MarkdownDelete, MarkdownEmphasis, MarkdownInlineCode, MarkdownStrong, MarkdownText,
};

#[derive(Debug)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum MarkdownLinkChild {
    Text(MarkdownText),
    Strong(MarkdownStrong),
    Emphasis(MarkdownEmphasis),
    Delete(MarkdownDelete),
    InlineCode(MarkdownInlineCode),
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("todo")]
    Text(#[from] crate::markdown::ast::text::ConvertError),
    #[error("todo")]
    Strong(#[from] crate::markdown::ast::strong::ConvertError),
    #[error("todo")]
    Emphasis(#[from] crate::markdown::ast::emphasis::ConvertError),
    #[error("todo")]
    Delete(#[from] crate::markdown::ast::delete::ConvertError),
    #[error("todo")]
    InlineCode(#[from] crate::markdown::ast::inline_code::ConvertError),
    #[error("todo")]
    InvalidNode(markdown::mdast::Node),
}

impl TryFrom<markdown::mdast::Node> for MarkdownLinkChild {
    type Error = ConvertError;

    fn try_from(value: markdown::mdast::Node) -> Result<Self, Self::Error> {
        use markdown::mdast::Node as N;

        Ok(match value {
            N::Text(txt) => Self::Text(txt.try_into()?),
            N::Strong(s) => Self::Strong(s.try_into()?),
            N::Emphasis(em) => Self::Emphasis(em.try_into()?),
            N::Delete(del) => Self::Delete(del.try_into()?),
            N::InlineCode(ic) => Self::InlineCode(ic.try_into()?),
            _ => return Err(ConvertError::InvalidNode(value)),
        })
    }
}
