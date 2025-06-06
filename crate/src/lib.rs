#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

pub mod markdown;

#[cfg(feature = "extism")]
#[cfg_attr(feature = "extism", extism_pdk::plugin_fn)]
pub fn parse_markdown(
    extism_pdk::Prost(request): extism_pdk::Prost<crate::markdown::pb::MarkdownServiceParseRequest>,
) -> extism_pdk::FnResult<extism_pdk::Prost<crate::markdown::pb::MarkdownServiceParseResponse>> {
    Ok(extism_pdk::Prost(
        crate::markdown::parse(&request.text).into(),
    ))
}

#[cfg(feature = "uniffi")]
#[cfg_attr(feature = "uniffi", uniffi::export)]
fn parse_markdown(text: &str) -> crate::markdown::ast::MarkdownRoot {
    crate::markdown::parse(text)
}
