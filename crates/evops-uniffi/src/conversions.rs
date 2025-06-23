impl From<evops_models::UserNameError> for crate::ValidateUserNameResult {
    fn from(value: evops_models::UserNameError) -> Self {
        use crate::ValidateUserNameResult as B;
        use evops_models::UserNameError as N;

        match value {
            N::LenCharMaxViolated => B::LenCharMaxViolated,
            N::NotEmptyViolated => B::NotEmptyViolated,
        }
    }
}

impl From<evops_models::EventTitleError> for crate::ValidateEventTitleResult {
    fn from(value: evops_models::EventTitleError) -> Self {
        use crate::ValidateEventTitleResult as B;
        use evops_models::EventTitleError as N;

        match value {
            N::LenCharMaxViolated => B::LenCharMaxViolated,
            N::NotEmptyViolated => B::NotEmptyViolated,
        }
    }
}

impl From<evops_models::EventDescriptionError> for crate::ValidateEventDescriptionResult {
    fn from(value: evops_models::EventDescriptionError) -> Self {
        use crate::ValidateEventDescriptionResult as B;
        use evops_models::EventDescriptionError as N;

        match value {
            N::LenCharMaxViolated => B::LenCharMaxViolated,
            N::NotEmptyViolated => B::NotEmptyViolated,
        }
    }
}

impl From<evops_models::TagNameError> for crate::ValidateTagNameResult {
    fn from(value: evops_models::TagNameError) -> Self {
        use crate::ValidateTagNameResult as B;
        use evops_models::TagNameError as N;

        match value {
            N::LenCharMaxViolated => B::LenCharMaxViolated,
            N::NotEmptyViolated => B::NotEmptyViolated,
            N::RegexViolated => B::RegexViolated,
        }
    }
}

impl From<evops_models::TagAliasError> for crate::ValidateTagAliasResult {
    fn from(value: evops_models::TagAliasError) -> Self {
        use crate::ValidateTagAliasResult as B;
        use evops_models::TagAliasError as N;

        match value {
            N::LenCharMaxViolated => B::LenCharMaxViolated,
            N::NotEmptyViolated => B::NotEmptyViolated,
        }
    }
}
