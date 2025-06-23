impl From<evops_models::UserNameError> for crate::ValidateUserNameResult {
    fn from(value: evops_models::UserNameError) -> Self {
        use crate::validate_user_name_result::Result as B;
        use evops_models::UserNameError as N;

        Self {
            result: Some(match value {
                N::LenCharMaxViolated => B::LenCharMaxViolated(()),
                N::NotEmptyViolated => B::NotEmptyViolated(()),
            }),
        }
    }
}

impl From<evops_models::EventTitleError> for crate::ValidateEventTitleResult {
    fn from(value: evops_models::EventTitleError) -> Self {
        use crate::validate_event_title_result::Result as B;
        use evops_models::EventTitleError as N;

        Self {
            result: Some(match value {
                N::LenCharMaxViolated => B::LenCharMaxViolated(()),
                N::NotEmptyViolated => B::NotEmptyViolated(()),
            }),
        }
    }
}

impl From<evops_models::EventDescriptionError> for crate::ValidateEventDescriptionResult {
    fn from(value: evops_models::EventDescriptionError) -> Self {
        use crate::validate_event_description_result::Result as B;
        use evops_models::EventDescriptionError as N;

        Self {
            result: Some(match value {
                N::LenCharMaxViolated => B::LenCharMaxViolated(()),
                N::NotEmptyViolated => B::NotEmptyViolated(()),
            }),
        }
    }
}

impl From<evops_models::TagNameError> for crate::ValidateTagNameResult {
    fn from(value: evops_models::TagNameError) -> Self {
        use crate::validate_tag_name_result::Result as B;
        use evops_models::TagNameError as N;

        Self {
            result: Some(match value {
                N::LenCharMaxViolated => B::LenCharMaxViolated(()),
                N::NotEmptyViolated => B::NotEmptyViolated(()),
                N::RegexViolated => B::RegexViolated(()),
            }),
        }
    }
}

impl From<evops_models::TagAliasError> for crate::ValidateTagAliasResult {
    fn from(value: evops_models::TagAliasError) -> Self {
        use crate::validate_tag_alias_result::Result as B;
        use evops_models::TagAliasError as N;

        Self {
            result: Some(match value {
                N::LenCharMaxViolated => B::LenCharMaxViolated(()),
                N::NotEmptyViolated => B::NotEmptyViolated(()),
            }),
        }
    }
}
