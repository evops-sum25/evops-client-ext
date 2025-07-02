#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

use markdown::{Constructs, ParseOptions};

use crate::ast::MarkdownRoot;

pub mod ast;
pub mod unist;

#[cfg(feature = "protobuf")]
mod pb_conversions;

/// # Panics
///
/// This function should never panic because Markdown does not have syntax errors.
#[must_use]
pub fn parse(text: &str) -> MarkdownRoot {
    let mdast = {
        markdown::to_mdast(text, &parse_options()).expect("CommonMark doesn't have syntax errors")
    };
    let markdown::mdast::Node::Root(root) = mdast else {
        unreachable!("when parsing entire documents, we should get a Root element");
    };

    MarkdownRoot::try_from(root)
        .expect("document parsed with ParseOptions::default should be CommonMark-compliant")
}

#[must_use]
#[inline]
pub fn parse_options() -> ParseOptions {
    ParseOptions {
        constructs: Constructs {
            definition: false,
            gfm_autolink_literal: true,
            gfm_strikethrough: true,
            html_flow: false,
            html_text: false,
            label_start_image: false,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        MarkdownParagraph, MarkdownParagraphChild, MarkdownRoot, MarkdownRootChild, MarkdownText,
    };
    use crate::unist::{MarkdownCoordinate, MarkdownPoint, MarkdownPosition};

    #[test]
    fn markdown() {
        assert_eq!(
            crate::parse(""),
            MarkdownRoot {
                children: Vec::default(),
                position: MarkdownPosition {
                    start: MarkdownPoint {
                        line: MarkdownCoordinate(1),
                        column: MarkdownCoordinate(1),
                        offset: MarkdownCoordinate(0),
                    },
                    end: MarkdownPoint {
                        line: MarkdownCoordinate(1),
                        column: MarkdownCoordinate(1),
                        offset: MarkdownCoordinate(0),
                    },
                },
            },
        );

        assert_eq!(
            crate::parse("Hello, world!"),
            MarkdownRoot {
                children: vec![MarkdownRootChild::Paragraph(MarkdownParagraph {
                    children: vec![MarkdownParagraphChild::Text(MarkdownText {
                        value: "Hello, world!".to_owned(),
                        position: MarkdownPosition {
                            start: MarkdownPoint {
                                line: MarkdownCoordinate(1),
                                column: MarkdownCoordinate(1),
                                offset: MarkdownCoordinate(0),
                            },
                            end: MarkdownPoint {
                                line: MarkdownCoordinate(1),
                                column: MarkdownCoordinate(14),
                                offset: MarkdownCoordinate(13),
                            },
                        },
                    })],
                    position: MarkdownPosition {
                        start: MarkdownPoint {
                            line: MarkdownCoordinate(1),
                            column: MarkdownCoordinate(1),
                            offset: MarkdownCoordinate(0),
                        },
                        end: MarkdownPoint {
                            line: MarkdownCoordinate(1),
                            column: MarkdownCoordinate(14),
                            offset: MarkdownCoordinate(13),
                        },
                    },
                })],
                position: MarkdownPosition {
                    start: MarkdownPoint {
                        line: MarkdownCoordinate(1),
                        column: MarkdownCoordinate(1),
                        offset: MarkdownCoordinate(0),
                    },
                    end: MarkdownPoint {
                        line: MarkdownCoordinate(1),
                        column: MarkdownCoordinate(14),
                        offset: MarkdownCoordinate(13),
                    },
                },
            },
        );
    }
}
