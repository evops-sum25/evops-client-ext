use nutype::nutype;

#[nutype(
    new_unchecked,
    validate(greater_or_equal = 0),
    derive(Debug, PartialEq, Eq, AsRef, Hash, Into)
)]
pub struct PgLimit(i64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pg_limit() {
        assert_eq!(
            PgLimit::try_new(i64::MIN),
            Err(PgLimitError::GreaterOrEqualViolated),
        );
        assert_eq!(
            PgLimit::try_new(-3),
            Err(PgLimitError::GreaterOrEqualViolated),
        );
        assert_eq!(
            PgLimit::try_new(-1),
            Err(PgLimitError::GreaterOrEqualViolated),
        );
        assert!(PgLimit::try_new(0).is_ok());
        assert!(PgLimit::try_new(52).is_ok());
        assert!(PgLimit::try_new(i64::MAX).is_ok());
    }
}
