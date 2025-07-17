use nutype::nutype;
use uuid::Uuid;

#[cfg(feature = "image")]
pub mod image;

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

#[nutype(
    new_unchecked,
    validate(predicate = |tag_ids| tag_ids.len() <= crate::EVENT_MAX_TAGS),
    derive(Debug, AsRef),
)]
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
pub struct UpdateEventForm {
    pub title: Option<crate::EventTitle>,
    pub description: Option<crate::EventDescription>,
    pub tag_ids: Option<crate::EventTagIds>,
    pub track_attendance: Option<bool>,
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
    #[cfg(feature = "chrono")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[cfg(feature = "chrono")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
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
