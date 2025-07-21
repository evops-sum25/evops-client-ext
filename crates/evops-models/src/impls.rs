use std::sync::LazyLock;

use regex::Regex;

use crate::{
    EventDescription, EventImageIds, EventTagIds, EventTags, EventTitle, LanguageName, TagAlias,
    TagAliases, TagName, UserDisplayName, UserLogin, UserPassword,
};

impl EventTags {
    pub const ITEMS_MAX: usize = 10;
}

impl EventTagIds {
    pub const ITEMS_MAX: usize = EventTags::ITEMS_MAX;
}

impl UserLogin {
    pub const LEN_CHAR_MIN: usize = 4;
    pub const LEN_CHAR_MAX: usize = 32;
    pub const REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("^[a-zA-Z][a-zA-Z0-9_]+$").unwrap());
}

impl UserDisplayName {
    pub const LEN_CHAR_MIN: usize = 1;
    pub const LEN_CHAR_MAX: usize = 64;
}

impl UserPassword {
    pub const LEN_CHAR_MIN: usize = 8;
    pub const LEN_CHAR_MAX: usize = 64;
    pub const REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"^[a-zA-Z0-9~`!@#$%^&*()\-_+={}\[\]|\\;:"<>,./?]+$"#).unwrap()
    });
}

impl LanguageName {
    pub const LEN_CHAR_MIN: usize = 1;
    pub const LEN_CHAR_MAX: usize = 32;
}

impl TagAlias {
    pub const LEN_CHAR_MIN: usize = 1;
    pub const LEN_CHAR_MAX: usize = 32;
}

impl TagAliases {
    pub const ITEMS_MAX: usize = 50;
}

impl TagName {
    pub const LEN_CHAR_MIN: usize = 1;
    pub const LEN_CHAR_MAX: usize = 32;
    pub const REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("^[a-z0-9][a-z0-9-]*$").unwrap());
}

impl EventDescription {
    pub const LEN_CHAR_MIN: usize = 1;
    pub const LEN_CHAR_MAX: usize = 5000;
}

impl EventTitle {
    pub const LEN_CHAR_MIN: usize = 1;
    pub const LEN_CHAR_MAX: usize = 64;
}

impl EventImageIds {
    pub const ITEMS_MAX: usize = 10;
}
