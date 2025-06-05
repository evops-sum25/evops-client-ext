include!(concat!(env!("OUT_DIR"), "/evops.markdown.v1.rs"));

impl From<crate::markdown::ast::Root> for self::ParseResponse {
    fn from(value: crate::markdown::ast::Root) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::markdown::unist::Point> for self::Point {
    fn from(value: crate::markdown::unist::Point) -> Self {
        const MSG: &str = "coordinate should fit in u32";

        Self {
            line: value.line.0.try_into().expect(MSG),
            column: value.column.0.try_into().expect(MSG),
            offset: value.offset.0.try_into().expect(MSG),
        }
    }
}

impl From<crate::markdown::unist::Position> for self::Position {
    fn from(value: crate::markdown::unist::Position) -> Self {
        Self {
            start: Some(value.start.into()),
            end: Some(value.end.into()),
        }
    }
}

impl From<crate::markdown::ast::RootChild> for self::RootChild {
    fn from(value: crate::markdown::ast::RootChild) -> Self {
        use crate::markdown::ast::RootChild as C;

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

impl From<crate::markdown::ast::Paragraph> for self::Paragraph {
    fn from(value: crate::markdown::ast::Paragraph) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::markdown::ast::ParagraphChild> for self::ParagraphChild {
    fn from(value: crate::markdown::ast::ParagraphChild) -> Self {
        use crate::markdown::ast::ParagraphChild as C;

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

impl From<crate::markdown::ast::Text> for self::Text {
    fn from(value: crate::markdown::ast::Text) -> Self {
        Self {
            value: value.value,
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::markdown::ast::Strong> for self::Strong {
    fn from(value: crate::markdown::ast::Strong) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::markdown::ast::Emphasis> for self::Emphasis {
    fn from(value: crate::markdown::ast::Emphasis) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::markdown::ast::Delete> for self::Delete {
    fn from(value: crate::markdown::ast::Delete) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::markdown::ast::InlineCode> for self::InlineCode {
    fn from(value: crate::markdown::ast::InlineCode) -> Self {
        Self {
            value: value.value,
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::markdown::ast::Link> for self::Link {
    fn from(value: crate::markdown::ast::Link) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            url: value.url,
        }
    }
}

impl From<crate::markdown::ast::LinkChild> for self::LinkChild {
    fn from(value: crate::markdown::ast::LinkChild) -> Self {
        use crate::markdown::ast::LinkChild as C;

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

impl From<crate::markdown::ast::Heading> for self::Heading {
    fn from(value: crate::markdown::ast::Heading) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            depth: self::HeadingDepth::from(value.depth).into(),
        }
    }
}

impl From<crate::markdown::ast::HeadingDepth> for self::HeadingDepth {
    fn from(value: crate::markdown::ast::HeadingDepth) -> Self {
        use crate::markdown::ast::HeadingDepth as D;

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

impl From<crate::markdown::ast::List> for self::List {
    fn from(value: crate::markdown::ast::List) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            ordered: value.ordered,
            spread: value.spread,
        }
    }
}

impl From<crate::markdown::ast::ListItem> for self::ListItem {
    fn from(value: crate::markdown::ast::ListItem) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
            spread: value.spread,
        }
    }
}

impl From<crate::markdown::ast::Blockquote> for self::Blockquote {
    fn from(value: crate::markdown::ast::Blockquote) -> Self {
        Self {
            children: value.children.into_iter().map(Into::into).collect(),
            position: Some(value.position.into()),
        }
    }
}

impl From<crate::markdown::ast::Code> for self::Code {
    fn from(value: crate::markdown::ast::Code) -> Self {
        Self {
            value: value.value,
            position: Some(value.position.into()),
            lang: value.lang,
        }
    }
}

impl From<crate::markdown::ast::ThematicBreak> for self::ThematicBreak {
    fn from(value: crate::markdown::ast::ThematicBreak) -> Self {
        Self {
            position: Some(value.position.into()),
        }
    }
}
