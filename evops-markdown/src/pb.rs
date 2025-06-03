include!(concat!(env!("OUT_DIR"), "/evops.markdown.v1.rs"));

impl From<crate::ast::Root> for self::ParseResponse {
    fn from(value: crate::ast::Root) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::unist::Point> for self::Point {
    fn from(value: crate::unist::Point) -> Self {
        const MSG: &str = "coordinate should fit in u32";

        Self {
            line: value.line.0.try_into().expect(MSG),
            column: value.column.0.try_into().expect(MSG),
            offset: value.offset.0.try_into().expect(MSG),
        }
    }
}

impl From<crate::unist::Position> for self::Position {
    fn from(value: crate::unist::Position) -> Self {
        Self {
            start: Some(value.start.into()),
            end: Some(value.end.into()),
        }
    }
}

impl From<crate::ast::RootChild> for self::RootChild {
    fn from(value: crate::ast::RootChild) -> Self {
        use crate::ast::RootChild as C;

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

impl From<crate::ast::Paragraph> for self::Paragraph {
    fn from(value: crate::ast::Paragraph) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::ast::ParagraphChild> for self::ParagraphChild {
    fn from(value: crate::ast::ParagraphChild) -> Self {
        use crate::ast::ParagraphChild as C;

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

impl From<crate::ast::Text> for self::Text {
    fn from(value: crate::ast::Text) -> Self {
        Self {
            value: value.value,
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::ast::Strong> for self::Strong {
    fn from(value: crate::ast::Strong) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::ast::Emphasis> for self::Emphasis {
    fn from(value: crate::ast::Emphasis) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::ast::Delete> for self::Delete {
    fn from(value: crate::ast::Delete) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::ast::InlineCode> for self::InlineCode {
    fn from(value: crate::ast::InlineCode) -> Self {
        Self {
            value: value.value,
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::ast::Link> for self::Link {
    fn from(value: crate::ast::Link) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            url: value.url,
        }
    }
}

impl From<crate::ast::LinkChild> for self::LinkChild {
    fn from(value: crate::ast::LinkChild) -> Self {
        use crate::ast::LinkChild as C;

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

impl From<crate::ast::Heading> for self::Heading {
    fn from(value: crate::ast::Heading) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            depth: self::HeadingDepth::from(value.depth).into(),
        }
    }
}

impl From<crate::ast::HeadingDepth> for self::HeadingDepth {
    fn from(value: crate::ast::HeadingDepth) -> Self {
        use crate::ast::HeadingDepth as D;

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

impl From<crate::ast::List> for self::List {
    fn from(value: crate::ast::List) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            ordered: value.ordered,
            spread: value.spread,
        }
    }
}

impl From<crate::ast::ListItem> for self::ListItem {
    fn from(value: crate::ast::ListItem) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            spread: value.spread,
        }
    }
}

impl From<crate::ast::Blockquote> for self::Blockquote {
    fn from(value: crate::ast::Blockquote) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::ast::Code> for self::Code {
    fn from(value: crate::ast::Code) -> Self {
        Self {
            value: value.value,
            position: Some(value.position.into()),
            lang: value.lang,
        }
    }
}

impl From<crate::ast::ThematicBreak> for self::ThematicBreak {
    fn from(value: crate::ast::ThematicBreak) -> Self {
        Self {
            position: Some(value.position.into()),
        }
    }
}
