impl<T> From<Result<T, evops_models::UserDisplayNameError>>
    for crate::ValidateUserDisplayNameResult
{
    fn from(value: Result<T, evops_models::UserDisplayNameError>) -> Self {
        use crate::validate_user_display_name_result::Result as B;
        use evops_models::UserDisplayNameError as N;

        Self {
            result: Some(match value {
                Ok(_) => B::Ok(()),
                Err(e) => match e {
                    N::LenCharMinViolated => B::LenCharMinViolated(()),
                    N::LenCharMaxViolated => B::LenCharMaxViolated(()),
                },
            }),
        }
    }
}

impl<T> From<Result<T, evops_models::UserLoginError>> for crate::ValidateUserLoginResult {
    fn from(value: Result<T, evops_models::UserLoginError>) -> Self {
        use crate::validate_user_login_result::Result as B;
        use evops_models::UserLoginError as N;

        Self {
            result: Some(match value {
                Ok(_) => B::Ok(()),
                Err(e) => match e {
                    N::LenCharMinViolated => B::LenCharMinViolated(()),
                    N::LenCharMaxViolated => B::LenCharMaxViolated(()),
                    N::RegexViolated => B::RegexViolated(()),
                },
            }),
        }
    }
}

impl<T> From<Result<T, evops_models::UserPasswordError>> for crate::ValidateUserPasswordResult {
    fn from(value: Result<T, evops_models::UserPasswordError>) -> Self {
        use crate::validate_user_password_result::Result as B;
        use evops_models::UserPasswordError as N;

        Self {
            result: Some(match value {
                Ok(_) => B::Ok(()),
                Err(e) => match e {
                    N::LenCharMinViolated => B::LenCharMinViolated(()),
                    N::LenCharMaxViolated => B::LenCharMaxViolated(()),
                    N::RegexViolated => B::RegexViolated(()),
                },
            }),
        }
    }
}

impl<T> From<Result<T, evops_models::EventTitleError>> for crate::ValidateEventTitleResult {
    fn from(value: Result<T, evops_models::EventTitleError>) -> Self {
        use crate::validate_event_title_result::Result as B;
        use evops_models::EventTitleError as N;

        Self {
            result: Some(match value {
                Ok(_) => B::Ok(()),
                Err(e) => match e {
                    N::LenCharMinViolated => B::LenCharMinViolated(()),
                    N::LenCharMaxViolated => B::LenCharMaxViolated(()),
                },
            }),
        }
    }
}

impl<T> From<Result<T, evops_models::EventDescriptionError>>
    for crate::ValidateEventDescriptionResult
{
    fn from(value: Result<T, evops_models::EventDescriptionError>) -> Self {
        use crate::validate_event_description_result::Result as B;
        use evops_models::EventDescriptionError as N;

        Self {
            result: Some(match value {
                Ok(_) => B::Ok(()),
                Err(e) => match e {
                    N::LenCharMinViolated => B::LenCharMinViolated(()),
                    N::LenCharMaxViolated => B::LenCharMaxViolated(()),
                },
            }),
        }
    }
}

impl<T> From<Result<T, evops_models::TagNameError>> for crate::ValidateTagNameResult {
    fn from(value: Result<T, evops_models::TagNameError>) -> Self {
        use crate::validate_tag_name_result::Result as B;
        use evops_models::TagNameError as N;

        Self {
            result: Some(match value {
                Ok(_) => B::Ok(()),
                Err(e) => match e {
                    N::LenCharMinViolated => B::LenCharMinViolated(()),
                    N::LenCharMaxViolated => B::LenCharMaxViolated(()),
                    N::RegexViolated => B::RegexViolated(()),
                },
            }),
        }
    }
}

impl<T> From<Result<T, evops_models::TagAliasError>> for crate::ValidateTagAliasResult {
    fn from(value: Result<T, evops_models::TagAliasError>) -> Self {
        use crate::validate_tag_alias_result::Result as B;
        use evops_models::TagAliasError as N;

        Self {
            result: Some(match value {
                Ok(_) => B::Ok(()),
                Err(e) => match e {
                    N::LenCharMinViolated => B::LenCharMinViolated(()),
                    N::LenCharMaxViolated => B::LenCharMaxViolated(()),
                },
            }),
        }
    }
}
