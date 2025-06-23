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
    match evops_models::UserName::try_new(user_name) {
        Ok(_) => self::ValidateUserNameResult::Ok,
        Err(e) => e.into(),
    }
}

#[derive(uniffi::Enum)]
enum ValidateEventTitleResult {
    Ok,
    LenCharMaxViolated,
    NotEmptyViolated,
}
#[uniffi::export]
fn validate_event_title(event_title: &str) -> self::ValidateEventTitleResult {
    match evops_models::EventTitle::try_new(event_title) {
        Ok(_) => self::ValidateEventTitleResult::Ok,
        Err(e) => e.into(),
    }
}

#[derive(uniffi::Enum)]
enum ValidateEventDescriptionResult {
    Ok,
    LenCharMaxViolated,
    NotEmptyViolated,
}
#[uniffi::export]
fn validate_event_description(event_description: &str) -> self::ValidateEventDescriptionResult {
    match evops_models::EventDescription::try_new(event_description) {
        Ok(_) => self::ValidateEventDescriptionResult::Ok,
        Err(e) => e.into(),
    }
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
    match evops_models::TagName::try_new(tag_name) {
        Ok(_) => self::ValidateTagNameResult::Ok,
        Err(e) => e.into(),
    }
}

#[derive(uniffi::Enum)]
enum ValidateTagAliasResult {
    Ok,
    LenCharMaxViolated,
    NotEmptyViolated,
}
#[uniffi::export]
fn validate_tag_alias(tag_alias: &str) -> ValidateTagAliasResult {
    match evops_models::TagAlias::try_new(tag_alias) {
        Ok(_) => self::ValidateTagAliasResult::Ok,
        Err(e) => e.into(),
    }
}
