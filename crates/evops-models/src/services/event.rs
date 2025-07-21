use nutype::nutype;
use uuid::Uuid;

use crate::{Tag, TagId};

#[cfg(feature = "image")]
pub mod image;

#[nutype(derive(Debug, Clone, Copy, Display))]
pub struct EventImageId(Uuid);

#[nutype(
    new_unchecked,
    validate(predicate = |image_urls| image_urls.len() <= Self::ITEMS_MAX),
    derive(Debug),
)]
pub struct EventImageIds(Vec<EventImageId>);

#[nutype(
    new_unchecked,
    validate(predicate = |tags| tags.len() <= Self::ITEMS_MAX),
    derive(Debug),
)]
pub struct EventTags(Vec<Tag>);

#[nutype(
    new_unchecked,
    validate(predicate = |tag_ids| tag_ids.len() <= Self::ITEMS_MAX),
    derive(Debug, AsRef),
)]
pub struct EventTagIds(Vec<TagId>);

#[derive(Debug)]
pub struct NewEventForm {
    pub title: EventTitle,
    pub description: EventDescription,
    pub tag_ids: EventTagIds,
}

#[derive(Debug)]
pub struct UpdateEventForm {
    pub title: Option<EventTitle>,
    pub description: Option<EventDescription>,
    pub tag_ids: Option<EventTagIds>,
}

#[cfg(feature = "chrono")]
#[derive(Debug)]
pub struct Event {
    pub id: EventId,
    pub title: EventTitle,
    pub description: EventDescription,
    pub author: crate::User,
    pub image_ids: EventImageIds,
    pub tags: EventTags,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display))]
pub struct EventId(Uuid);

#[nutype(
    new_unchecked,
    validate(len_char_min = EventTitle::LEN_CHAR_MIN, len_char_max = EventTitle::LEN_CHAR_MAX),
    derive(Debug, PartialEq, Eq, AsRef, Hash),
)]
pub struct EventTitle(String);

#[nutype(
    new_unchecked,
    validate(
        len_char_min = EventDescription::LEN_CHAR_MIN,
        len_char_max = EventDescription::LEN_CHAR_MAX,
    ),
    derive(Debug, PartialEq, Eq, AsRef, Hash)
)]
pub struct EventDescription(String);

#[allow(clippy::repeat_once)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_title() {
        assert_eq!(
            EventTitle::try_new(""),
            Err(EventTitleError::LenCharMinViolated),
        );
        assert!(EventTitle::try_new("a".repeat(1)).is_ok());
        assert!(EventTitle::try_new("a".repeat(64)).is_ok());
        assert_eq!(
            EventTitle::try_new("a".repeat(65)),
            Err(EventTitleError::LenCharMaxViolated),
        );
    }

    #[test]
    fn event_description() {
        assert_eq!(
            EventDescription::try_new(""),
            Err(EventDescriptionError::LenCharMinViolated),
        );
        assert!(EventDescription::try_new("a".repeat(1)).is_ok());
        assert!(EventDescription::try_new("a".repeat(5000)).is_ok());
        assert_eq!(
            EventDescription::try_new("a".repeat(5001)),
            Err(EventDescriptionError::LenCharMaxViolated),
        );
    }
}
