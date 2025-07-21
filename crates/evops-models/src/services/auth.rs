use nutype::nutype;
use uuid::Uuid;

#[cfg(feature = "chrono")]
mod jwt_numeric_date;

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

#[nutype(
    new_unchecked,
    validate(
        len_char_min = UserDisplayName::LEN_CHAR_MIN,
        len_char_max = UserDisplayName::LEN_CHAR_MAX,
    ),
    derive(Debug, PartialEq, Eq, AsRef, Hash),
)]
pub struct UserDisplayName(String);

#[nutype(
    new_unchecked,
    validate(
        len_char_min = UserLogin::LEN_CHAR_MIN,
        len_char_max = UserLogin::LEN_CHAR_MAX,
        regex = UserPassword::REGEX,
    ),
    derive(Debug, PartialEq, Eq, AsRef, Hash),
)]
pub struct UserLogin(String);

#[nutype(
    validate(
        len_char_min = UserPassword::LEN_CHAR_MIN,
        len_char_max = UserPassword::LEN_CHAR_MAX,
        regex = UserPassword::REGEX,
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
#[derive(serde::Serialize, serde::Deserialize)]
pub struct JwtClaims {
    pub sub: UserId,
    #[serde(with = "jwt_numeric_date")]
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
    use super::*;

    #[test]
    fn user_display_name() {
        assert_eq!(
            UserDisplayName::try_new(""),
            Err(UserDisplayNameError::LenCharMinViolated),
        );
        assert!(UserDisplayName::try_new("a".repeat(1)).is_ok());
        assert!(UserDisplayName::try_new("a".repeat(64)).is_ok());
        assert_eq!(
            UserDisplayName::try_new("a".repeat(65)),
            Err(UserDisplayNameError::LenCharMaxViolated),
        );
    }
}
