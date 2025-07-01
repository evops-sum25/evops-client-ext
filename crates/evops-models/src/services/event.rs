use std::io::Cursor;

use bytes::Bytes;
use chrono::{DateTime, Utc};
use image::{DynamicImage, ImageReader};
use nutype::nutype;
use uuid::Uuid;

use crate::ApiError;

#[nutype(validate(predicate = |_image| true), derive(Debug))]
pub struct EventImage(DynamicImage);

impl TryFrom<Bytes> for crate::EventImage {
    type Error = ApiError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let image_raw = {
            ImageReader::new(Cursor::new(value))
                .with_guessed_format()
                .map_err(|e| ApiError::Other(e.to_string()))?
                .decode()
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        };

        Self::try_new(image_raw).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

pub const EVENT_MAX_IMAGES: usize = 10;
#[nutype(derive(Debug, Clone, Copy, Display))]
pub struct EventImageId(Uuid);

#[nutype(
    new_unchecked,
    validate(predicate = |image_urls| image_urls.len() <= crate::EVENT_MAX_IMAGES),
    derive(Debug),
)]
pub struct EventImageIds(Vec<EventImageId>);

pub const EVENT_MAX_TAGS: usize = 10;

#[nutype(
    new_unchecked,
    validate(predicate = |tags| tags.len() <= crate::EVENT_MAX_TAGS),
    derive(Debug),
)]
pub struct EventTags(Vec<crate::Tag>);

#[nutype(validate(predicate = |tag_ids| tag_ids.len() <= crate::EVENT_MAX_TAGS), derive(Debug))]
pub struct EventTagIds(Vec<crate::TagId>);

#[derive(Debug)]
pub struct NewEventForm {
    pub title: crate::EventTitle,
    pub description: crate::EventDescription,
    pub author_id: crate::UserId,
    pub tag_ids: crate::EventTagIds,
    pub with_attendance: bool,
}

#[derive(Debug)]
pub struct Event {
    pub id: crate::EventId,
    pub title: crate::EventTitle,
    pub description: crate::EventDescription,
    pub author: crate::User,
    pub image_ids: EventImageIds,
    pub tags: crate::EventTags,
    pub with_attendance: bool,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display))]
pub struct EventId(Uuid);

pub const EVENT_TITLE_MIN_LEN: usize = 1;
pub const EVENT_TITLE_MAX_LEN: usize = 64;
#[nutype(
    new_unchecked,
    validate(len_char_max = crate::EVENT_TITLE_MAX_LEN, not_empty),
    derive(Debug, PartialEq, Eq, AsRef, Hash),
)]
pub struct EventTitle(String);

pub const EVENT_DESCRIPTION_MIN_LEN: usize = 1;
pub const EVENT_DESCRIPTION_MAX_LEN: usize = 5000;
#[nutype(
    new_unchecked,
    validate(len_char_max = crate::EVENT_DESCRIPTION_MAX_LEN, not_empty),
    derive(Debug, PartialEq, Eq, AsRef, Hash)
)]
pub struct EventDescription(String);

#[allow(clippy::repeat_once)]
#[cfg(test)]
mod tests {
    #[test]
    fn event_title() {
        assert_eq!(
            crate::EventTitle::try_new(""),
            Err(crate::EventTitleError::NotEmptyViolated),
        );
        assert!(crate::EventTitle::try_new("a".repeat(1)).is_ok());
        assert!(crate::EventTitle::try_new("a".repeat(64)).is_ok());
        assert_eq!(
            crate::EventTitle::try_new("a".repeat(65)),
            Err(crate::EventTitleError::LenCharMaxViolated),
        );
    }

    #[test]
    fn event_description() {
        assert_eq!(
            crate::EventDescription::try_new(""),
            Err(crate::EventDescriptionError::NotEmptyViolated),
        );
        assert!(crate::EventDescription::try_new("a".repeat(1)).is_ok());
        assert!(crate::EventDescription::try_new("a".repeat(5000)).is_ok());
        assert_eq!(
            crate::EventDescription::try_new("a".repeat(5001)),
            Err(crate::EventDescriptionError::LenCharMaxViolated),
        );
    }
}
