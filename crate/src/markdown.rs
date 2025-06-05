use markdown::{Constructs, ParseOptions};

pub mod ast;
pub mod unist;

#[cfg(feature = "protobuf")]
pub(crate) mod pb;

#[must_use]
pub fn parse(value: &str) -> self::ast::Root {
    let mdast = {
        markdown::to_mdast(value, &parse_options()).expect("CommonMark doesn't have syntax errors")
    };
    let markdown::mdast::Node::Root(root) = mdast else {
        unreachable!("when parsing entire documents, we should get a Root element");
    };

    self::ast::Root::try_from(root)
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
