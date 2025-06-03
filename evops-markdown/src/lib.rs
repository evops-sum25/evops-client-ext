#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

use markdown::{Constructs, ParseOptions};

pub mod ast;
#[cfg(feature = "protobuf")]
mod pb;
pub mod unist;

#[must_use]
pub fn parse(value: &str) -> crate::ast::Root {
    let mdast = {
        markdown::to_mdast(value, &parse_options()).expect("CommonMark doesn't have syntax errors")
    };
    let markdown::mdast::Node::Root(root) = mdast else {
        unreachable!("when parsing entire documents, we should get a Root element");
    };

    crate::ast::Root::try_from(root)
        .expect("document parsed with ParseOptions::default should be CommonMark-compliant")
}

#[cfg(feature = "extism")]
#[cfg_attr(feature = "extism", extism_pdk::plugin_fn)]
pub fn parse_markdown(
    extism_pdk::Prost(request): extism_pdk::Prost<crate::pb::ParseRequest>,
) -> extism_pdk::FnResult<extism_pdk::Prost<crate::pb::ParseResponse>> {
    Ok(extism_pdk::Prost(parse(&request.text).into()))
}

#[cfg(feature = "uniffi")]
#[cfg_attr(feature = "uniffi", uniffi::export)]
fn parse_markdown(value: &str) -> crate::ast::Root {
    parse(value)
}

#[must_use]
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
