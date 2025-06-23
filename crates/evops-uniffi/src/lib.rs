mod conversions;

uniffi::setup_scaffolding!();

#[uniffi::export]
fn parse_markdown(text: &str) -> evops_markdown::ast::MarkdownRoot {
    evops_markdown::parse(text)
}

#[derive(uniffi::Enum)]
enum ValidateUserNameResult {
    Ok,
    LenCharMaxViolated,
    NotEmptyViolated,
}
#[uniffi::export]
fn validate_user_name(user_name: &str) -> self::ValidateUserNameResult {
    evops_models::UserName::try_new(user_name).into()
}

#[derive(uniffi::Enum)]
enum ValidateEventTitleResult {
    Ok,
    LenCharMaxViolated,
    NotEmptyViolated,
}
#[uniffi::export]
fn validate_event_title(event_title: &str) -> self::ValidateEventTitleResult {
    evops_models::EventTitle::try_new(event_title).into()
}

#[derive(uniffi::Enum)]
enum ValidateEventDescriptionResult {
    Ok,
    LenCharMaxViolated,
    NotEmptyViolated,
}
#[uniffi::export]
fn validate_event_description(event_description: &str) -> self::ValidateEventDescriptionResult {
    evops_models::EventDescription::try_new(event_description).into()
}

#[derive(uniffi::Enum)]
enum ValidateTagNameResult {
    Ok,
    LenCharMaxViolated,
    NotEmptyViolated,
    RegexViolated,
}
#[uniffi::export]
fn validate_tag_name(tag_name: &str) -> ValidateTagNameResult {
    evops_models::TagName::try_new(tag_name).into()
}

#[derive(uniffi::Enum)]
enum ValidateTagAliasResult {
    Ok,
    LenCharMaxViolated,
    NotEmptyViolated,
}
#[uniffi::export]
fn validate_tag_alias(tag_alias: &str) -> ValidateTagAliasResult {
    evops_models::TagAlias::try_new(tag_alias).into()
}
