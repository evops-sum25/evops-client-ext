use nutype::nutype;
use uuid::Uuid;

#[nutype(
    new_unchecked,
    validate(predicate = |tag_aliases| tag_aliases.len() <= Self::ITEMS_MAX),
    derive(Debug, AsRef),
)]
pub struct TagAliases(Vec<TagAlias>);

#[derive(Debug)]
pub struct NewTagForm {
    pub name: TagName,
    pub aliases: TagAliases,
}

#[derive(Debug)]
pub struct Tag {
    pub id: TagId,
    pub name: TagName,
    pub aliases: TagAliases,
}

#[nutype(derive(Debug, Clone, Copy, Debug, PartialEq, Eq, Hash, Display))]
pub struct TagId(Uuid);

#[nutype(
    new_unchecked,
    validate(
        len_char_min = TagName::LEN_CHAR_MIN,
        len_char_max = TagName::LEN_CHAR_MAX,
        regex = TagName::REGEX,
    ),
    derive(Debug, PartialEq, Eq, AsRef, Hash, Display),
)]
pub struct TagName(String);

#[nutype(
    new_unchecked,
    validate(len_char_min = TagAlias::LEN_CHAR_MIN, len_char_max = TagAlias::LEN_CHAR_MAX),
    derive(Debug, PartialEq, Eq, AsRef, Hash, Display),
)]
pub struct TagAlias(String);

#[allow(clippy::repeat_once)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_name() {
        assert_eq!(TagName::try_new(""), Err(TagNameError::LenCharMinViolated),);
        assert!(TagName::try_new("a".repeat(1)).is_ok());
        assert!(TagName::try_new("a".repeat(32)).is_ok());
        assert_eq!(
            TagName::try_new("a".repeat(33)),
            Err(TagNameError::LenCharMaxViolated),
        );
        assert!(TagName::try_new("just-a-normal-tag").is_ok());
        assert!(TagName::try_new("a--111-821ar-st-b238---").is_ok());
        assert!(TagName::try_new("8-----").is_ok());
        assert!(TagName::try_new("88005553535").is_ok());
        assert_eq!(
            TagName::try_new("-a--111-821ar-st-b238---"),
            Err(TagNameError::RegexViolated),
        );
        assert_eq!(
            TagName::try_new("two words"),
            Err(TagNameError::RegexViolated),
        );
    }

    #[test]
    fn tag_alias() {
        assert_eq!(
            TagAlias::try_new(""),
            Err(TagAliasError::LenCharMinViolated),
        );
        assert!(TagAlias::try_new("a".repeat(1)).is_ok());
        assert!(TagAlias::try_new("a".repeat(32)).is_ok());
        assert_eq!(
            TagAlias::try_new("a".repeat(33)),
            Err(TagAliasError::LenCharMaxViolated),
        );
        assert!(TagAlias::try_new("just-a-normal-tag").is_ok());
        assert!(TagAlias::try_new("a--111-821ar-st-b238---").is_ok());
        assert!(TagAlias::try_new("8-----").is_ok());
        assert!(TagAlias::try_new("88005553535").is_ok());
    }
}
