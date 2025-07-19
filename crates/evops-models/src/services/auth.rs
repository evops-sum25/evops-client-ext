use std::sync::LazyLock;

use nutype::nutype;
use regex::Regex;
#[cfg(feature = "chrono")]
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

pub struct NewUserForm {
    pub login: UserLogin,
    pub display_name: UserDisplayName,
    pub password: UserPassword,
}

#[derive(Debug)]
pub struct User {
    pub id: UserId,
    pub login: UserLogin,
    pub display_name: UserDisplayName,
}

#[nutype(derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Display,
    Serialize,
    Deserialize,
))]
pub struct UserId(Uuid);

pub const USER_DISPLAY_NAME_MIN_LEN: usize = 1;
pub const USER_DISPLAY_NAME_MAX_LEN: usize = 64;
#[nutype(
    new_unchecked,
    validate(len_char_min = USER_DISPLAY_NAME_MIN_LEN, len_char_max = USER_DISPLAY_NAME_MAX_LEN),
    derive(Debug, PartialEq, Eq, AsRef, Hash),
)]
pub struct UserDisplayName(String);

pub const USER_LOGIN_MIN_LEN: usize = 4;
pub const USER_LOGIN_MAX_LEN: usize = 32;
pub static USER_LOGIN_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[a-zA-Z][a-zA-Z0-9_]+$").unwrap());
#[nutype(
    new_unchecked,
    validate(
        len_char_min = USER_LOGIN_MIN_LEN,
        len_char_max = USER_LOGIN_MAX_LEN,
        regex = USER_LOGIN_REGEX,
    ),
    derive(Debug, PartialEq, Eq, AsRef, Hash),
)]
pub struct UserLogin(String);

pub const USER_PASSWORD_MIN_LEN: usize = 8;
pub const USER_PASSWORD_MAX_LEN: usize = 64;
pub static USER_PASSWORD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^[a-zA-Z0-9~`!@#$%^&*()\-_+={}\[\]|\\;:"<>,./?]+$"#).unwrap());
#[nutype(
    validate(
        len_char_min = USER_PASSWORD_MIN_LEN,
        len_char_max = USER_PASSWORD_MAX_LEN,
        regex = USER_PASSWORD_REGEX,
    ),
    derive(AsRef),
)]
pub struct UserPassword(String);

#[nutype(derive(Debug, AsRef))]
pub struct UserPasswordHash(String);

#[nutype(derive(Debug, AsRef))]
pub struct JsonWebToken(String);

#[nutype(derive(Debug, AsRef))]
pub struct JsonWebTokenHash([u8; 32]);

#[cfg(feature = "chrono")]
#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: UserId,
    pub exp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct AuthTokens {
    pub access: JsonWebToken,
    pub refresh: JsonWebToken,
}

#[allow(clippy::repeat_once)]
#[cfg(test)]
mod tests {
    #[test]
    fn user_display_name() {
        assert_eq!(
            crate::UserDisplayName::try_new(""),
            Err(crate::UserDisplayNameError::LenCharMinViolated),
        );
        assert!(crate::UserDisplayName::try_new("a".repeat(1)).is_ok());
        assert!(crate::UserDisplayName::try_new("a".repeat(64)).is_ok());
        assert_eq!(
            crate::UserDisplayName::try_new("a".repeat(65)),
            Err(crate::UserDisplayNameError::LenCharMaxViolated),
        );
    }
}
