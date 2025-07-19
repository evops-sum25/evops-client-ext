//! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

/// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let timestamp = date.timestamp();
    serializer.serialize_i64(timestamp)
}

/// Attempts to deserialize an i64 and use as a Unix timestamp
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    DateTime::from_timestamp(i64::deserialize(deserializer)?, 0)
        .ok_or(serde::de::Error::custom("invalid Unix timestamp value"))
}
