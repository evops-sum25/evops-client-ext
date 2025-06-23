use extism_pdk::{FnResult, Prost};

use evops_pb_ext::{MarkdownServiceParseRequest, MarkdownServiceParseResponse};

#[extism_pdk::plugin_fn]
pub fn parse_markdown(
    Prost(request): Prost<MarkdownServiceParseRequest>,
) -> FnResult<Prost<MarkdownServiceParseResponse>> {
    Ok(Prost(MarkdownServiceParseResponse {
        root: Some(evops_markdown::parse(&request.text).into()),
    }))
}
