use extism_pdk::{FnResult, Prost};

use evops_pb_ext::{
    MarkdownRoot, ValidateEventDescriptionResult, ValidateEventTitleResult, ValidateTagAliasResult,
    ValidateTagNameResult, ValidateUserNameResult,
};

#[extism_pdk::plugin_fn]
pub fn parse_markdown(text: String) -> FnResult<Prost<MarkdownRoot>> {
    Ok(Prost(evops_markdown::parse(&text).into()))
}

#[extism_pdk::plugin_fn]
pub fn validate_user_name(user_name: String) -> FnResult<Prost<ValidateUserNameResult>> {
    Ok(Prost(evops_models::UserName::try_new(user_name).into()))
}

#[extism_pdk::plugin_fn]
pub fn validate_event_title(event_title: String) -> FnResult<Prost<ValidateEventTitleResult>> {
    Ok(Prost(evops_models::EventTitle::try_new(event_title).into()))
}

#[extism_pdk::plugin_fn]
pub fn validate_event_description(
    event_description: String,
) -> FnResult<Prost<ValidateEventDescriptionResult>> {
    Ok(Prost({
        evops_models::EventDescription::try_new(event_description).into()
    }))
}

#[extism_pdk::plugin_fn]
pub fn validate_tag_name(tag_name: String) -> FnResult<Prost<ValidateTagNameResult>> {
    Ok(Prost(evops_models::TagName::try_new(tag_name).into()))
}

#[extism_pdk::plugin_fn]
pub fn validate_tag_alias(tag_alias: String) -> FnResult<Prost<ValidateTagAliasResult>> {
    Ok(Prost(evops_models::TagAlias::try_new(tag_alias).into()))
}
