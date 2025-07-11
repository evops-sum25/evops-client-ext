use nutype::nutype;
use uuid::Uuid;

#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display))]
pub struct LanguageId(Uuid);

pub const LANGUAGE_MIN_LEN: usize = 1;
pub const LANGUAGE_MAX_LEN: usize = 32;

#[nutype(new_unchecked, validate(len_char_max = LANGUAGE_MAX_LEN, not_empty), derive(Debug))]
pub struct LanguageName(String);

pub struct Language {
    pub id: LanguageId,
    pub name: LanguageName,
}
