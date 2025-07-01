use nutype::nutype;
use uuid::Uuid;

#[derive(Debug)]
pub struct NewUserForm {
    pub name: crate::UserName,
}

#[derive(Debug)]
pub struct User {
    pub id: crate::UserId,
    pub name: crate::UserName,
}

#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display))]
pub struct UserId(Uuid);

pub const USER_NAME_MIN_LEN: usize = 1;
pub const USER_NAME_MAX_LEN: usize = 64;
#[nutype(
    new_unchecked,
    validate(len_char_max = crate::USER_NAME_MAX_LEN, not_empty),
    derive(Debug, PartialEq, Eq, AsRef, Hash),
)]
pub struct UserName(String);

#[allow(clippy::repeat_once)]
#[cfg(test)]
mod tests {
    #[test]
    fn user_name() {
        assert_eq!(
            crate::UserName::try_new(""),
            Err(crate::UserNameError::NotEmptyViolated),
        );
        assert!(crate::UserName::try_new("a".repeat(1)).is_ok());
        assert!(crate::UserName::try_new("a".repeat(64)).is_ok());
        assert_eq!(
            crate::UserName::try_new("a".repeat(65)),
            Err(crate::UserNameError::LenCharMaxViolated),
        );
    }
}
