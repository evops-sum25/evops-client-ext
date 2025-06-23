use extism_pdk::{FnResult, Prost};

use evops_markdown::pb::{MarkdownServiceParseRequest, MarkdownServiceParseResponse};

#[extism_pdk::plugin_fn]
pub fn parse_markdown(
    Prost(request): Prost<MarkdownServiceParseRequest>,
) -> FnResult<Prost<MarkdownServiceParseResponse>> {
    Ok(Prost(MarkdownServiceParseResponse {
        root: Some(evops_markdown::parse(&request.text).into()),
    }))
}
