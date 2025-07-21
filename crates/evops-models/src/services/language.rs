use nutype::nutype;
use uuid::Uuid;

#[nutype(
    new_unchecked,
    validate(
        len_char_min = LanguageName::LEN_CHAR_MIN,
        len_char_max = LanguageName::LEN_CHAR_MAX,
    ),
    derive(Debug),
)]
pub struct LanguageName(String);

#[nutype(derive(Debug))]
pub struct LanguageId(Uuid);

pub struct NewLanguageForm {
    pub name: LanguageName,
}
