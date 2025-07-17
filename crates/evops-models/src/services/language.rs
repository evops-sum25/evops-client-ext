use nutype::nutype;
use uuid::Uuid;

pub const LANGUAGE_NAME_MAX_LEN: usize = 32;

#[nutype(new_unchecked, validate(len_char_max = LANGUAGE_NAME_MAX_LEN, not_empty), derive(Debug))]
pub struct LanguageName(String);

#[nutype(derive(Debug))]
pub struct LanguageId(Uuid);

pub struct NewLanguageForm {
    pub name: LanguageName,
}
