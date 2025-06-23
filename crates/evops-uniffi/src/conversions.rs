impl<T> From<Result<T, evops_models::UserNameError>> for crate::ValidateUserNameResult {
    fn from(value: Result<T, evops_models::UserNameError>) -> Self {
        use crate::ValidateUserNameResult as B;
        use evops_models::UserNameError as N;
        match value {
            Ok(_) => B::Ok,
            Err(e) => match e {
                N::LenCharMaxViolated => B::LenCharMaxViolated,
                N::NotEmptyViolated => B::NotEmptyViolated,
            },
        }
    }
}

impl<T> From<Result<T, evops_models::EventTitleError>> for crate::ValidateEventTitleResult {
    fn from(value: Result<T, evops_models::EventTitleError>) -> Self {
        use crate::ValidateEventTitleResult as B;
        use evops_models::EventTitleError as N;

        match value {
            Ok(_) => B::Ok,
            Err(e) => match e {
                N::LenCharMaxViolated => B::LenCharMaxViolated,
                N::NotEmptyViolated => B::NotEmptyViolated,
            },
        }
    }
}

impl<T> From<Result<T, evops_models::EventDescriptionError>>
    for crate::ValidateEventDescriptionResult
{
    fn from(value: Result<T, evops_models::EventDescriptionError>) -> Self {
        use crate::ValidateEventDescriptionResult as B;
        use evops_models::EventDescriptionError as N;

        match value {
            Ok(_) => B::Ok,
            Err(e) => match e {
                N::LenCharMaxViolated => B::LenCharMaxViolated,
                N::NotEmptyViolated => B::NotEmptyViolated,
            },
        }
    }
}

impl<T> From<Result<T, evops_models::TagNameError>> for crate::ValidateTagNameResult {
    fn from(value: Result<T, evops_models::TagNameError>) -> Self {
        use crate::ValidateTagNameResult as B;
        use evops_models::TagNameError as N;

        match value {
            Ok(_) => B::Ok,
            Err(e) => match e {
                N::LenCharMaxViolated => B::LenCharMaxViolated,
                N::NotEmptyViolated => B::NotEmptyViolated,
                N::RegexViolated => B::RegexViolated,
            },
        }
    }
}

impl<T> From<Result<T, evops_models::TagAliasError>> for crate::ValidateTagAliasResult {
    fn from(value: Result<T, evops_models::TagAliasError>) -> Self {
        use crate::ValidateTagAliasResult as B;
        use evops_models::TagAliasError as N;

        match value {
            Ok(_) => B::Ok,
            Err(e) => match e {
                N::LenCharMaxViolated => B::LenCharMaxViolated,
                N::NotEmptyViolated => B::NotEmptyViolated,
            },
        }
    }
}
