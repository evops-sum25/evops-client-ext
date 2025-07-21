impl<T> From<Result<T, evops_models::UserDisplayNameError>> for crate::ValidateUserNameResult {
    fn from(value: Result<T, evops_models::UserDisplayNameError>) -> Self {
        match value {
            Ok(_) => Self::Ok,
            Err(e) => {
                use evops_models::UserDisplayNameError as E;
                match e {
                    E::LenCharMinViolated => Self::LenCharMinViolated,
                    E::LenCharMaxViolated => Self::LenCharMaxViolated,
                }
            }
        }
    }
}

impl<T> From<Result<T, evops_models::EventTitleError>> for crate::ValidateEventTitleResult {
    fn from(value: Result<T, evops_models::EventTitleError>) -> Self {
        match value {
            Ok(_) => Self::Ok,
            Err(e) => {
                use evops_models::EventTitleError as E;
                match e {
                    E::LenCharMinViolated => Self::LenCharMinViolated,
                    E::LenCharMaxViolated => Self::LenCharMaxViolated,
                }
            }
        }
    }
}

impl<T> From<Result<T, evops_models::EventDescriptionError>>
    for crate::ValidateEventDescriptionResult
{
    fn from(value: Result<T, evops_models::EventDescriptionError>) -> Self {
        match value {
            Ok(_) => Self::Ok,
            Err(e) => {
                use evops_models::EventDescriptionError as E;
                match e {
                    E::LenCharMinViolated => Self::LenCharMinViolated,
                    E::LenCharMaxViolated => Self::LenCharMaxViolated,
                }
            }
        }
    }
}

impl<T> From<Result<T, evops_models::TagNameError>> for crate::ValidateTagNameResult {
    fn from(value: Result<T, evops_models::TagNameError>) -> Self {
        match value {
            Ok(_) => Self::Ok,
            Err(e) => {
                use evops_models::TagNameError as E;
                match e {
                    E::LenCharMinViolated => Self::LenCharMinViolated,
                    E::LenCharMaxViolated => Self::LenCharMaxViolated,
                    E::RegexViolated => Self::RegexViolated,
                }
            }
        }
    }
}

impl<T> From<Result<T, evops_models::UserLoginError>> for crate::ValidateUserLoginResult {
    fn from(value: Result<T, evops_models::UserLoginError>) -> Self {
        use evops_models::UserLoginError as N;

        match value {
            Ok(_) => Self::Ok,
            Err(e) => match e {
                N::LenCharMinViolated => Self::LenCharMinViolated,
                N::LenCharMaxViolated => Self::LenCharMaxViolated,
                N::RegexViolated => Self::RegexViolated,
            },
        }
    }
}

impl<T> From<Result<T, evops_models::UserPasswordError>> for crate::ValidateUserPasswordResult {
    fn from(value: Result<T, evops_models::UserPasswordError>) -> Self {
        use evops_models::UserPasswordError as N;

        match value {
            Ok(_) => Self::Ok,
            Err(e) => match e {
                N::LenCharMinViolated => Self::LenCharMinViolated,
                N::LenCharMaxViolated => Self::LenCharMaxViolated,
                N::RegexViolated => Self::RegexViolated,
            },
        }
    }
}

impl<T> From<Result<T, evops_models::TagAliasError>> for crate::ValidateTagAliasResult {
    fn from(value: Result<T, evops_models::TagAliasError>) -> Self {
        use evops_models::TagAliasError as N;

        match value {
            Ok(_) => Self::Ok,
            Err(e) => match e {
                N::LenCharMinViolated => Self::LenCharMinViolated,
                N::LenCharMaxViolated => Self::LenCharMaxViolated,
            },
        }
    }
}
