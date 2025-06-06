include!(concat!(env!("OUT_DIR"), "/evops.markdown.v1.rs"));

mod native {
    pub use crate::markdown::ast::{
        MarkdownBlockquote, MarkdownCode, MarkdownDelete, MarkdownEmphasis, MarkdownHeading,
        MarkdownHeadingDepth, MarkdownInlineCode, MarkdownLink, MarkdownLinkChild, MarkdownList,
        MarkdownListItem, MarkdownParagraph, MarkdownParagraphChild, MarkdownRoot,
        MarkdownRootChild, MarkdownStrong, MarkdownText, MarkdownThematicBreak,
    };
    pub use crate::markdown::unist::{MarkdownPoint, MarkdownPosition};
}

impl From<self::native::MarkdownRoot> for self::MarkdownServiceParseResponse {
    fn from(value: self::native::MarkdownRoot) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<self::native::MarkdownPoint> for self::MarkdownPoint {
    fn from(value: self::native::MarkdownPoint) -> Self {
        const MSG: &str = "coordinate should fit in u32";

        Self {
            line: value.line.0.try_into().expect(MSG),
            column: value.column.0.try_into().expect(MSG),
            offset: value.offset.0.try_into().expect(MSG),
        }
    }
}

impl From<self::native::MarkdownPosition> for self::MarkdownPosition {
    fn from(value: self::native::MarkdownPosition) -> Self {
        Self {
            start: Some(value.start.into()),
            end: Some(value.end.into()),
        }
    }
}

impl From<self::native::MarkdownRootChild> for self::MarkdownRootChild {
    fn from(value: self::native::MarkdownRootChild) -> Self {
        use self::markdown_root_child::Child as Pb;
        use self::native::MarkdownRootChild as N;

        Self {
            child: Some(match value {
                N::Paragraph(p) => Pb::Paragraph(p.into()),
                N::Heading(h) => Pb::Heading(h.into()),
                N::List(ls) => Pb::List(ls.into()),
                N::Blockquote(bq) => Pb::Blockquote(bq.into()),
                N::Code(c) => Pb::Code(c.into()),
                N::ThematicBreak(tb) => Pb::ThematicBreak(tb.into()),
            }),
        }
    }
}

impl From<self::native::MarkdownParagraph> for self::MarkdownParagraph {
    fn from(value: self::native::MarkdownParagraph) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<self::native::MarkdownParagraphChild> for self::MarkdownParagraphChild {
    fn from(value: self::native::MarkdownParagraphChild) -> Self {
        use self::markdown_paragraph_child::Child as Pb;
        use self::native::MarkdownParagraphChild as N;

        Self {
            child: Some(match value {
                N::Text(txt) => Pb::Text(txt.into()),
                N::Strong(s) => Pb::Strong(s.into()),
                N::Emphasis(em) => Pb::Emphasis(em.into()),
                N::Delete(del) => Pb::Delete(del.into()),
                N::InlineCode(ic) => Pb::InlineCode(ic.into()),
                N::Link(ln) => Pb::Link(ln.into()),
            }),
        }
    }
}

impl From<self::native::MarkdownText> for self::MarkdownText {
    fn from(value: self::native::MarkdownText) -> Self {
        Self {
            value: value.value,
            position: Some(value.position.into()),
        }
    }
}

impl From<self::native::MarkdownStrong> for self::MarkdownStrong {
    fn from(value: self::native::MarkdownStrong) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<self::native::MarkdownEmphasis> for self::MarkdownEmphasis {
    fn from(value: self::native::MarkdownEmphasis) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<self::native::MarkdownDelete> for self::MarkdownDelete {
    fn from(value: self::native::MarkdownDelete) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<self::native::MarkdownInlineCode> for self::MarkdownInlineCode {
    fn from(value: self::native::MarkdownInlineCode) -> Self {
        Self {
            value: value.value,
            position: Some(value.position.into()),
        }
    }
}

impl From<self::native::MarkdownLink> for self::MarkdownLink {
    fn from(value: self::native::MarkdownLink) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            url: value.url,
        }
    }
}

impl From<self::native::MarkdownLinkChild> for self::MarkdownLinkChild {
    fn from(value: self::native::MarkdownLinkChild) -> Self {
        use self::markdown_link_child::Child as Pb;
        use self::native::MarkdownLinkChild as N;

        Self {
            child: Some(match value {
                N::Text(txt) => Pb::Text(txt.into()),
                N::Strong(s) => Pb::Strong(s.into()),
                N::Emphasis(em) => Pb::Emphasis(em.into()),
                N::Delete(del) => Pb::Delete(del.into()),
                N::InlineCode(ic) => Pb::InlineCode(ic.into()),
            }),
        }
    }
}

impl From<self::native::MarkdownHeading> for self::MarkdownHeading {
    fn from(value: self::native::MarkdownHeading) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            depth: self::MarkdownHeadingDepth::from(value.depth).into(),
        }
    }
}

impl From<self::native::MarkdownHeadingDepth> for self::MarkdownHeadingDepth {
    fn from(value: self::native::MarkdownHeadingDepth) -> Self {
        use self::native::MarkdownHeadingDepth as D;

        match value {
            D::Level1 => Self::MarkdownHeadingDepth1,
            D::Level2 => Self::MarkdownHeadingDepth2,
            D::Level3 => Self::MarkdownHeadingDepth3,
            D::Level4 => Self::MarkdownHeadingDepth4,
            D::Level5 => Self::MarkdownHeadingDepth5,
            D::Level6 => Self::MarkdownHeadingDepth6,
        }
    }
}

impl From<self::native::MarkdownList> for self::MarkdownList {
    fn from(value: self::native::MarkdownList) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            ordered: value.ordered,
            spread: value.spread,
        }
    }
}

impl From<self::native::MarkdownListItem> for self::MarkdownListItem {
    fn from(value: self::native::MarkdownListItem) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            spread: value.spread,
        }
    }
}

impl From<self::native::MarkdownBlockquote> for self::MarkdownBlockquote {
    fn from(value: self::native::MarkdownBlockquote) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<self::native::MarkdownCode> for self::MarkdownCode {
    fn from(value: self::native::MarkdownCode) -> Self {
        Self {
            value: value.value,
            position: Some(value.position.into()),
            lang: value.lang,
        }
    }
}

impl From<self::native::MarkdownThematicBreak> for self::MarkdownThematicBreak {
    fn from(value: self::native::MarkdownThematicBreak) -> Self {
        Self {
            position: Some(value.position.into()),
        }
    }
}
