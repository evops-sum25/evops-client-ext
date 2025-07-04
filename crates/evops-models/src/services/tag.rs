use std::sync::LazyLock;

use nutype::nutype;
use regex::Regex;
use uuid::Uuid;

pub const TAG_MAX_ALIASES: usize = 50;
#[nutype(
    new_unchecked,
    validate(predicate = |tag_aliases| tag_aliases.len() <= crate::TAG_MAX_ALIASES),
    derive(Debug, AsRef),
)]
pub struct TagAliases(Vec<crate::TagAlias>);

#[derive(Debug)]
pub struct NewTagForm {
    pub name: crate::TagName,
    pub aliases: TagAliases,
}

#[derive(Debug)]
pub struct Tag {
    pub id: crate::TagId,
    pub name: crate::TagName,
    pub aliases: TagAliases,
}

#[nutype(derive(Debug, Clone, Copy, Debug, PartialEq, Eq, Hash, Display))]
pub struct TagId(Uuid);

pub static TAG_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[a-z0-9][a-z0-9-]*$").unwrap());
pub const TAG_NAME_MIN_LEN: usize = 1;
pub const TAG_NAME_MAX_LEN: usize = 32;
#[nutype(
    new_unchecked,
    validate(len_char_max = TAG_NAME_MAX_LEN, not_empty, regex = TAG_NAME_REGEX),
    derive(Debug, PartialEq, Eq, AsRef, Hash, Display),
)]
pub struct TagName(String);

pub const TAG_ALIAS_MIN_LEN: usize = 1;
pub const TAG_ALIAS_MAX_LEN: usize = TAG_NAME_MAX_LEN;
#[nutype(
    new_unchecked,
    validate(len_char_max = TAG_ALIAS_MAX_LEN, not_empty),
    derive(Debug, PartialEq, Eq, AsRef, Hash, Display),
)]
pub struct TagAlias(String);

#[allow(clippy::repeat_once)]
#[cfg(test)]
mod tests {
    #[test]
    fn tag_name() {
        assert_eq!(
            crate::TagName::try_new(""),
            Err(crate::TagNameError::NotEmptyViolated),
        );
        assert!(crate::TagName::try_new("a".repeat(1)).is_ok());
        assert!(crate::TagName::try_new("a".repeat(32)).is_ok());
        assert_eq!(
            crate::TagName::try_new("a".repeat(33)),
            Err(crate::TagNameError::LenCharMaxViolated),
        );
        assert!(crate::TagName::try_new("just-a-normal-tag").is_ok());
        assert!(crate::TagName::try_new("a--111-821ar-st-b238---").is_ok());
        assert!(crate::TagName::try_new("8-----").is_ok());
        assert!(crate::TagName::try_new("88005553535").is_ok());
        assert_eq!(
            crate::TagName::try_new("-a--111-821ar-st-b238---"),
            Err(crate::TagNameError::RegexViolated),
        );
        assert_eq!(
            crate::TagName::try_new("two words"),
            Err(crate::TagNameError::RegexViolated),
        );
    }

    #[test]
    fn tag_alias() {
        assert_eq!(
            crate::TagAlias::try_new(""),
            Err(crate::TagAliasError::NotEmptyViolated),
        );
        assert!(crate::TagAlias::try_new("a".repeat(1)).is_ok());
        assert!(crate::TagAlias::try_new("a".repeat(32)).is_ok());
        assert_eq!(
            crate::TagAlias::try_new("a".repeat(33)),
            Err(crate::TagAliasError::LenCharMaxViolated),
        );
        assert!(crate::TagAlias::try_new("just-a-normal-tag").is_ok());
        assert!(crate::TagAlias::try_new("a--111-821ar-st-b238---").is_ok());
        assert!(crate::TagAlias::try_new("8-----").is_ok());
        assert!(crate::TagAlias::try_new("88005553535").is_ok());
    }
}
