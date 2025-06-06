use crate::markdown::ast::{
    MarkdownBlockquote, MarkdownCode, MarkdownDelete, MarkdownEmphasis, MarkdownHeading,
    MarkdownHeadingDepth, MarkdownInlineCode, MarkdownLink, MarkdownLinkChild, MarkdownList,
    MarkdownListItem, MarkdownParagraph, MarkdownParagraphChild, MarkdownRoot, MarkdownRootChild,
    MarkdownStrong, MarkdownText, MarkdownThematicBreak,
};
use crate::markdown::unist::{MarkdownPoint, MarkdownPosition};

include!(concat!(env!("OUT_DIR"), "/evops.markdown.v1.rs"));

impl From<MarkdownRoot> for self::ParseResponse {
    fn from(value: MarkdownRoot) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<MarkdownPoint> for self::Point {
    fn from(value: MarkdownPoint) -> Self {
        const MSG: &str = "coordinate should fit in u32";

        Self {
            line: value.line.0.try_into().expect(MSG),
            column: value.column.0.try_into().expect(MSG),
            offset: value.offset.0.try_into().expect(MSG),
        }
    }
}

impl From<MarkdownPosition> for self::Position {
    fn from(value: MarkdownPosition) -> Self {
        Self {
            start: Some(value.start.into()),
            end: Some(value.end.into()),
        }
    }
}

impl From<MarkdownRootChild> for self::RootChild {
    fn from(value: MarkdownRootChild) -> Self {
        use MarkdownRootChild as C;

        Self {
            child: Some(match value {
                C::Paragraph(p) => self::root_child::Child::Paragraph(p.into()),
                C::Heading(h) => self::root_child::Child::Heading(h.into()),
                C::List(ls) => self::root_child::Child::List(ls.into()),
                C::Blockquote(bq) => self::root_child::Child::Blockquote(bq.into()),
                C::Code(c) => self::root_child::Child::Code(c.into()),
                C::ThematicBreak(tb) => self::root_child::Child::ThematicBreak(tb.into()),
            }),
        }
    }
}

impl From<MarkdownParagraph> for self::Paragraph {
    fn from(value: MarkdownParagraph) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<MarkdownParagraphChild> for self::ParagraphChild {
    fn from(value: MarkdownParagraphChild) -> Self {
        use MarkdownParagraphChild as C;

        Self {
            child: Some(match value {
                C::Text(txt) => self::paragraph_child::Child::Text(txt.into()),
                C::Strong(s) => self::paragraph_child::Child::Strong(s.into()),
                C::Emphasis(em) => self::paragraph_child::Child::Emphasis(em.into()),
                C::Delete(del) => self::paragraph_child::Child::Delete(del.into()),
                C::InlineCode(ic) => self::paragraph_child::Child::InlineCode(ic.into()),
                C::Link(ln) => self::paragraph_child::Child::Link(ln.into()),
            }),
        }
    }
}

impl From<MarkdownText> for self::Text {
    fn from(value: MarkdownText) -> Self {
        Self {
            value: value.value,
            position: Some(value.position.into()),
        }
    }
}

impl From<MarkdownStrong> for self::Strong {
    fn from(value: MarkdownStrong) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<MarkdownEmphasis> for self::Emphasis {
    fn from(value: MarkdownEmphasis) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<MarkdownDelete> for self::Delete {
    fn from(value: MarkdownDelete) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<MarkdownInlineCode> for self::InlineCode {
    fn from(value: MarkdownInlineCode) -> Self {
        Self {
            value: value.value,
            position: Some(value.position.into()),
        }
    }
}

impl From<MarkdownLink> for self::Link {
    fn from(value: MarkdownLink) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            url: value.url,
        }
    }
}

impl From<MarkdownLinkChild> for self::LinkChild {
    fn from(value: MarkdownLinkChild) -> Self {
        use MarkdownLinkChild as C;

        Self {
            child: Some(match value {
                C::Text(txt) => self::link_child::Child::Text(txt.into()),
                C::Strong(s) => self::link_child::Child::Strong(s.into()),
                C::Emphasis(em) => self::link_child::Child::Emphasis(em.into()),
                C::Delete(del) => self::link_child::Child::Delete(del.into()),
                C::InlineCode(ic) => self::link_child::Child::InlineCode(ic.into()),
            }),
        }
    }
}

impl From<MarkdownHeading> for self::Heading {
    fn from(value: MarkdownHeading) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            depth: self::HeadingDepth::from(value.depth).into(),
        }
    }
}

impl From<MarkdownHeadingDepth> for self::HeadingDepth {
    fn from(value: MarkdownHeadingDepth) -> Self {
        use MarkdownHeadingDepth as D;

        match value {
            D::Level1 => Self::HeadingDepth1,
            D::Level2 => Self::HeadingDepth2,
            D::Level3 => Self::HeadingDepth3,
            D::Level4 => Self::HeadingDepth4,
            D::Level5 => Self::HeadingDepth5,
            D::Level6 => Self::HeadingDepth6,
        }
    }
}

impl From<MarkdownList> for self::List {
    fn from(value: MarkdownList) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            ordered: value.ordered,
            spread: value.spread,
        }
    }
}

impl From<MarkdownListItem> for self::ListItem {
    fn from(value: MarkdownListItem) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            spread: value.spread,
        }
    }
}

impl From<MarkdownBlockquote> for self::Blockquote {
    fn from(value: MarkdownBlockquote) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<MarkdownCode> for self::Code {
    fn from(value: MarkdownCode) -> Self {
        Self {
            value: value.value,
            position: Some(value.position.into()),
            lang: value.lang,
        }
    }
}

impl From<MarkdownThematicBreak> for self::ThematicBreak {
    fn from(value: MarkdownThematicBreak) -> Self {
        Self {
            position: Some(value.position.into()),
        }
    }
}
