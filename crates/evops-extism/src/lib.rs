#![allow(clippy::unnecessary_wraps)]

use extism_pdk::{FnResult, Json, Prost};

use evops_pb_ext::{
    MarkdownRoot, ValidateEventDescriptionResult, ValidateEventTitleResult, ValidateTagAliasResult,
    ValidateTagNameResult, ValidateUserDisplayNameResult, ValidateUserLoginResult,
    ValidateUserPasswordResult,
};

#[extism_pdk::plugin_fn]
pub fn get_event_description_len_char_max() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::EventDescription::LEN_CHAR_MAX))
}

#[extism_pdk::plugin_fn]
pub fn get_event_description_len_char_min() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::EventDescription::LEN_CHAR_MIN))
}

#[extism_pdk::plugin_fn]
pub fn get_event_title_len_char_max() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::EventTitle::LEN_CHAR_MAX))
}

#[extism_pdk::plugin_fn]
pub fn get_event_title_len_char_min() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::EventTitle::LEN_CHAR_MIN))
}

#[extism_pdk::plugin_fn]
pub fn get_tag_alias_len_char_max() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::TagAlias::LEN_CHAR_MAX))
}

#[extism_pdk::plugin_fn]
pub fn get_tag_alias_len_char_min() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::TagAlias::LEN_CHAR_MIN))
}

#[extism_pdk::plugin_fn]
pub fn get_tag_name_len_char_max() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::TagName::LEN_CHAR_MAX))
}

#[extism_pdk::plugin_fn]
pub fn get_tag_name_len_char_min() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::TagName::LEN_CHAR_MIN))
}

#[extism_pdk::plugin_fn]
pub fn get_tag_name_regex() -> FnResult<Json<String>> {
    Ok(Json(evops_models::TagName::REGEX.to_string()))
}

#[extism_pdk::plugin_fn]
pub fn get_user_display_name_len_char_max() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::UserDisplayName::LEN_CHAR_MAX))
}

#[extism_pdk::plugin_fn]
pub fn get_user_display_name_len_char_min() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::UserDisplayName::LEN_CHAR_MIN))
}

#[extism_pdk::plugin_fn]
pub fn get_user_login_len_char_max() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::UserLogin::LEN_CHAR_MAX))
}

#[extism_pdk::plugin_fn]
pub fn get_user_login_len_char_min() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::UserLogin::LEN_CHAR_MIN))
}

#[extism_pdk::plugin_fn]
pub fn get_user_login_regex() -> FnResult<Json<String>> {
    Ok(Json(evops_models::UserLogin::REGEX.to_string()))
}

#[extism_pdk::plugin_fn]
pub fn get_user_password_len_char_max() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::UserPassword::LEN_CHAR_MAX))
}

#[extism_pdk::plugin_fn]
pub fn get_user_password_len_char_min() -> FnResult<Json<usize>> {
    Ok(Json(evops_models::UserPassword::LEN_CHAR_MIN))
}

#[extism_pdk::plugin_fn]
pub fn get_user_password_regex() -> FnResult<Json<String>> {
    Ok(Json(evops_models::UserPassword::REGEX.to_string()))
}

#[extism_pdk::plugin_fn]
pub fn parse_markdown(text: String) -> FnResult<Prost<MarkdownRoot>> {
    Ok(Prost(evops_markdown::parse(&text).into()))
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
pub fn validate_event_title(event_title: String) -> FnResult<Prost<ValidateEventTitleResult>> {
    Ok(Prost(evops_models::EventTitle::try_new(event_title).into()))
}

#[extism_pdk::plugin_fn]
pub fn validate_tag_alias(tag_alias: String) -> FnResult<Prost<ValidateTagAliasResult>> {
    Ok(Prost(evops_models::TagAlias::try_new(tag_alias).into()))
}

#[extism_pdk::plugin_fn]
pub fn validate_tag_name(tag_name: String) -> FnResult<Prost<ValidateTagNameResult>> {
    Ok(Prost(evops_models::TagName::try_new(tag_name).into()))
}

#[extism_pdk::plugin_fn]
pub fn validate_user_display_name(
    user_name: String,
) -> FnResult<Prost<ValidateUserDisplayNameResult>> {
    Ok(Prost(
        evops_models::UserDisplayName::try_new(user_name).into(),
    ))
}

#[extism_pdk::plugin_fn]
pub fn validate_user_login(user_login: String) -> FnResult<Prost<ValidateUserLoginResult>> {
    Ok(Prost(evops_models::UserLogin::try_new(user_login).into()))
}

#[extism_pdk::plugin_fn]
pub fn validate_user_password(
    user_password: String,
) -> FnResult<Prost<ValidateUserPasswordResult>> {
    Ok(Prost(
        evops_models::UserPassword::try_new(user_password).into(),
    ))
}
