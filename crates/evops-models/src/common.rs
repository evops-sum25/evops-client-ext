use nutype::nutype;

#[nutype(
    new_unchecked,
    validate(greater_or_equal = 0),
    derive(Debug, PartialEq, Eq, AsRef, Hash, Into)
)]
pub struct PgLimit(i64);

#[cfg(test)]
mod tests {
    #[test]
    fn pg_limit() {
        assert_eq!(
            crate::PgLimit::try_new(i64::MIN),
            Err(crate::PgLimitError::GreaterOrEqualViolated),
        );
        assert_eq!(
            crate::PgLimit::try_new(-3),
            Err(crate::PgLimitError::GreaterOrEqualViolated),
        );
        assert_eq!(
            crate::PgLimit::try_new(-1),
            Err(crate::PgLimitError::GreaterOrEqualViolated),
        );
        assert!(crate::PgLimit::try_new(0).is_ok());
        assert!(crate::PgLimit::try_new(52).is_ok());
        assert!(crate::PgLimit::try_new(i64::MAX).is_ok());
    }
}
