use chrono::{DateTime, Utc};
use nutype::nutype;
use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct EventServiceFindRequest {
    pub id: crate::EventId,
}

#[derive(Debug)]
pub struct EventServiceFindResponse {
    pub event: crate::Event,
}

#[derive(Debug)]
pub struct EventServiceListRequest {
    pub last_id: Option<crate::EventId>,
    pub limit: Option<crate::PgLimit>, // CONSIDER: create separate type for it?
}

#[derive(Debug)]
pub struct EventServiceListResponse {
    pub events: Vec<crate::Event>,
}

#[derive(Debug)]
pub struct EventServiceCreateRequest {
    pub form: crate::NewEventForm,
}

#[derive(Debug)]
pub struct EventServiceCreateResponse {
    pub event: crate::Event,
}

#[derive(Debug)]
pub struct NewEventForm {
    pub author_id: crate::UserId,
    pub image_urls: Option<Vec<Url>>,
    pub title: crate::EventTitle,
    pub description: crate::EventDescription,
    pub tag_ids: Option<Vec<crate::TagId>>,
    pub with_attendance: bool,
}

#[derive(Debug)]
pub struct Event {
    pub id: crate::EventId,
    pub author: crate::User,
    pub image_urls: Vec<Url>,
    pub title: crate::EventTitle,
    pub description: crate::EventDescription,
    pub tags: Vec<crate::Tag>,
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
