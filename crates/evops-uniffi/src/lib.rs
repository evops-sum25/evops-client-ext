mod conversions;

uniffi::setup_scaffolding!();

#[uniffi::export]
fn get_event_description_len_char_max() -> u16 {
    evops_models::EventDescription::LEN_CHAR_MAX
        .try_into()
        .unwrap()
}

#[uniffi::export]
fn get_event_description_len_char_min() -> u16 {
    evops_models::EventDescription::LEN_CHAR_MIN
        .try_into()
        .unwrap()
}

#[uniffi::export]
fn get_event_title_len_char_max() -> u8 {
    evops_models::EventTitle::LEN_CHAR_MAX.try_into().unwrap()
}

#[uniffi::export]
fn get_event_title_len_char_min() -> u8 {
    evops_models::EventTitle::LEN_CHAR_MIN.try_into().unwrap()
}

#[uniffi::export]
fn get_tag_alias_len_char_max() -> u8 {
    evops_models::TagAlias::LEN_CHAR_MAX.try_into().unwrap()
}

#[uniffi::export]
fn get_tag_alias_len_char_min() -> u8 {
    evops_models::TagAlias::LEN_CHAR_MIN.try_into().unwrap()
}

#[uniffi::export]
fn get_tag_name_len_char_max() -> u8 {
    evops_models::TagName::LEN_CHAR_MAX.try_into().unwrap()
}

#[uniffi::export]
fn get_tag_name_len_char_min() -> u8 {
    evops_models::TagName::LEN_CHAR_MIN.try_into().unwrap()
}

#[uniffi::export]
fn get_tag_name_regex() -> String {
    evops_models::TagName::REGEX.to_string()
}

#[uniffi::export]
fn get_user_display_name_len_char_max() -> u8 {
    evops_models::UserDisplayName::LEN_CHAR_MAX
        .try_into()
        .unwrap()
}

#[uniffi::export]
fn get_user_display_name_len_char_min() -> u8 {
    evops_models::UserDisplayName::LEN_CHAR_MIN
        .try_into()
        .unwrap()
}

#[uniffi::export]
fn get_user_login_len_char_max() -> u8 {
    evops_models::UserLogin::LEN_CHAR_MAX.try_into().unwrap()
}

#[uniffi::export]
fn get_user_login_len_char_min() -> u8 {
    evops_models::UserLogin::LEN_CHAR_MIN.try_into().unwrap()
}

#[uniffi::export]
fn get_user_login_regex() -> String {
    evops_models::UserLogin::REGEX.to_string()
}

#[uniffi::export]
fn get_user_password_len_char_max() -> u8 {
    evops_models::UserPassword::LEN_CHAR_MAX.try_into().unwrap()
}

#[uniffi::export]
fn get_user_password_len_char_min() -> u8 {
    evops_models::UserPassword::LEN_CHAR_MIN.try_into().unwrap()
}

#[uniffi::export]
fn get_user_password_regex() -> String {
    evops_models::UserPassword::REGEX.to_string()
}

#[uniffi::export]
fn parse_markdown(text: &str) -> evops_markdown::ast::MarkdownRoot {
    evops_markdown::parse(text)
}

#[derive(uniffi::Enum)]
enum ValidateEventDescriptionResult {
    Ok,
    LenCharMinViolated,
    LenCharMaxViolated,
}
#[uniffi::export]
fn validate_event_description(event_description: &str) -> self::ValidateEventDescriptionResult {
    evops_models::EventDescription::try_new(event_description).into()
}

#[derive(uniffi::Enum)]
enum ValidateEventTitleResult {
    Ok,
    LenCharMinViolated,
    LenCharMaxViolated,
}
#[uniffi::export]
fn validate_event_title(event_title: &str) -> self::ValidateEventTitleResult {
    evops_models::EventTitle::try_new(event_title).into()
}

#[derive(uniffi::Enum)]
enum ValidateTagAliasResult {
    Ok,
    LenCharMinViolated,
    LenCharMaxViolated,
}
#[uniffi::export]
fn validate_tag_alias(tag_alias: &str) -> ValidateTagAliasResult {
    evops_models::TagAlias::try_new(tag_alias).into()
}

#[derive(uniffi::Enum)]
enum ValidateTagNameResult {
    Ok,
    LenCharMinViolated,
    LenCharMaxViolated,
    RegexViolated,
}
#[uniffi::export]
fn validate_tag_name(tag_name: &str) -> ValidateTagNameResult {
    evops_models::TagName::try_new(tag_name).into()
}

#[derive(uniffi::Enum)]
enum ValidateUserDisplayNameResult {
    Ok,
    LenCharMinViolated,
    LenCharMaxViolated,
}
#[uniffi::export]
fn validate_user_display_name(user_name: &str) -> self::ValidateUserDisplayNameResult {
    evops_models::UserDisplayName::try_new(user_name).into()
}

#[derive(uniffi::Enum)]
enum ValidateUserLoginResult {
    Ok,
    LenCharMinViolated,
    LenCharMaxViolated,
    RegexViolated,
}
#[uniffi::export]
fn validate_user_login(user_login: &str) -> ValidateUserLoginResult {
    evops_models::UserLogin::try_new(user_login).into()
}

#[derive(uniffi::Enum)]
enum ValidateUserPasswordResult {
    Ok,
    LenCharMinViolated,
    LenCharMaxViolated,
    RegexViolated,
}
#[uniffi::export]
fn validate_user_password(user_password: &str) -> ValidateUserPasswordResult {
    evops_models::UserPassword::try_new(user_password).into()
}
