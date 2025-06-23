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
impl From<evops_models::UserNameError> for ValidateUserNameResult {
    fn from(value: evops_models::UserNameError) -> Self {
        use self::ValidateUserNameResult as B;
        use evops_models::UserNameError as N;

        match value {
            N::LenCharMaxViolated => B::LenCharMaxViolated,
            N::NotEmptyViolated => B::NotEmptyViolated,
        }
    }
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
impl From<evops_models::EventTitleError> for ValidateEventTitleResult {
    fn from(value: evops_models::EventTitleError) -> Self {
        use self::ValidateEventTitleResult as B;
        use evops_models::EventTitleError as N;

        match value {
            N::LenCharMaxViolated => B::LenCharMaxViolated,
            N::NotEmptyViolated => B::NotEmptyViolated,
        }
    }
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
impl From<evops_models::EventDescriptionError> for self::ValidateEventDescriptionResult {
    fn from(value: evops_models::EventDescriptionError) -> Self {
        use self::ValidateEventDescriptionResult as B;
        use evops_models::EventDescriptionError as N;

        match value {
            N::LenCharMaxViolated => B::LenCharMaxViolated,
            N::NotEmptyViolated => B::NotEmptyViolated,
        }
    }
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
impl From<evops_models::TagNameError> for ValidateTagNameResult {
    fn from(value: evops_models::TagNameError) -> Self {
        use self::ValidateTagNameResult as B;
        use evops_models::TagNameError as N;

        match value {
            N::LenCharMaxViolated => B::LenCharMaxViolated,
            N::NotEmptyViolated => B::NotEmptyViolated,
            N::RegexViolated => B::RegexViolated,
        }
    }
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
    RegexViolated,
}
impl From<evops_models::TagAliasError> for ValidateTagAliasResult {
    fn from(value: evops_models::TagAliasError) -> Self {
        use self::ValidateTagAliasResult as B;
        use evops_models::TagAliasError as N;

        match value {
            N::LenCharMaxViolated => B::LenCharMaxViolated,
            N::NotEmptyViolated => B::NotEmptyViolated,
        }
    }
}
#[uniffi::export]
fn validate_tag_alias(tag_alias: &str) -> ValidateTagAliasResult {
    match evops_models::TagAlias::try_new(tag_alias) {
        Ok(_) => self::ValidateTagAliasResult::Ok,
        Err(e) => e.into(),
    }
}
