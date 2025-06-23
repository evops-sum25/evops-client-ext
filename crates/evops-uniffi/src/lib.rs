uniffi::setup_scaffolding!();

#[uniffi::export]
fn parse_markdown(text: &str) -> evops_markdown::ast::MarkdownServiceParseResponse {
    evops_markdown::parse(text)
}
