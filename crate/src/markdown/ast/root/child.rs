use thiserror::Error;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum RootChild {
    Paragraph(crate::markdown::ast::Paragraph),
    Heading(crate::markdown::ast::Heading),
    List(crate::markdown::ast::List),
    Blockquote(crate::markdown::ast::Blockquote),
    Code(crate::markdown::ast::Code),
    ThematicBreak(crate::markdown::ast::ThematicBreak),
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("todo")]
    Paragraph(#[from] crate::markdown::ast::paragraph::ConvertError),
    #[error("todo")]
    Heading(#[from] crate::markdown::ast::heading::ConvertError),
    #[error("todo")]
    List(#[from] crate::markdown::ast::list::ConvertError),
    #[error("todo")]
    Blockquote(#[from] crate::markdown::ast::blockquote::ConvertError),
    #[error("todo")]
    Code(#[from] crate::markdown::ast::code::ConvertError),
    #[error("todo")]
    ThematicBreak(#[from] crate::markdown::ast::thematic_break::ConvertError),
    #[error("todo")]
    InvalidNode(markdown::mdast::Node),
}

impl TryFrom<markdown::mdast::Node> for RootChild {
    type Error = ConvertError;

    fn try_from(value: markdown::mdast::Node) -> Result<Self, Self::Error> {
        use markdown::mdast::Node as N;

        Ok(match value {
            N::Paragraph(p) => Self::Paragraph(p.try_into()?),
            N::Heading(h) => Self::Heading(h.try_into()?),
            N::List(ls) => Self::List(ls.try_into()?),
            N::Blockquote(bq) => Self::Blockquote(bq.try_into()?),
            N::Code(it) => Self::Code(it.try_into()?),
            N::ThematicBreak(tb) => Self::ThematicBreak(tb.try_into()?),
            _ => {
                return Err(ConvertError::InvalidNode(value));
            }
        })
    }
}
