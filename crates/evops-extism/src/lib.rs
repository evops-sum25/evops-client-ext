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
    Ok(Prost(match evops_models::UserName::try_new(user_name) {
        Ok(_) => ValidateUserNameResult {
            result: Some(evops_pb_ext::validate_user_name_result::Result::Ok(())),
        },
        Err(e) => e.into(),
    }))
}

#[extism_pdk::plugin_fn]
pub fn validate_event_title(event_title: String) -> FnResult<Prost<ValidateEventTitleResult>> {
    Ok(Prost(
        match evops_models::EventTitle::try_new(event_title) {
            Ok(_) => ValidateEventTitleResult {
                result: Some(evops_pb_ext::validate_event_title_result::Result::Ok(())),
            },
            Err(e) => e.into(),
        },
    ))
}

#[extism_pdk::plugin_fn]
pub fn validate_event_description(
    event_description: String,
) -> FnResult<Prost<ValidateEventDescriptionResult>> {
    Ok(Prost(
        match evops_models::EventDescription::try_new(event_description) {
            Ok(_) => ValidateEventDescriptionResult {
                result: Some(evops_pb_ext::validate_event_description_result::Result::Ok(
                    (),
                )),
            },
            Err(e) => e.into(),
        },
    ))
}

#[extism_pdk::plugin_fn]
pub fn validate_tag_name(tag_name: String) -> FnResult<Prost<ValidateTagNameResult>> {
    Ok(Prost(match evops_models::TagName::try_new(tag_name) {
        Ok(_) => ValidateTagNameResult {
            result: Some(evops_pb_ext::validate_tag_name_result::Result::Ok(())),
        },
        Err(e) => e.into(),
    }))
}

#[extism_pdk::plugin_fn]
pub fn validate_tag_alias(tag_alias: String) -> FnResult<Prost<ValidateTagAliasResult>> {
    Ok(Prost(match evops_models::TagAlias::try_new(tag_alias) {
        Ok(_) => ValidateTagAliasResult {
            result: Some(evops_pb_ext::validate_tag_alias_result::Result::Ok(())),
        },
        Err(e) => e.into(),
    }))
}
