use nutype::nutype;

#[nutype(
    new_unchecked,
    validate(greater_or_equal = 0),
    derive(Debug, PartialEq, Eq, AsRef, Hash, Into)
)]
pub struct PgLimit(i64);