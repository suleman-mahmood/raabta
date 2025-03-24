use uuid::Uuid;

use crate::routes::{
    admin_portal::announcement_route::AnnouncementPostData,
    api::announcement_route::CreateAnnoucementBody,
};

pub struct NewAnnouncement {
    pub id: Uuid,
    pub announcement: String,
    pub announcer_id: String,
    pub class_id: Option<String>,
}

impl TryFrom<CreateAnnoucementBody> for NewAnnouncement {
    type Error = String;

    fn try_from(value: CreateAnnoucementBody) -> Result<Self, Self::Error> {
        if value.announcement.trim().is_empty() {
            return Err("Annoucement cannot be empty".to_string());
        }

        Ok(Self {
            id: Uuid::new_v4(),
            announcement: value.announcement,
            announcer_id: value.announcer_id,
            class_id: value.class_id,
        })
    }
}

impl TryFrom<AnnouncementPostData> for NewAnnouncement {
    type Error = String;

    fn try_from(value: AnnouncementPostData) -> Result<Self, Self::Error> {
        if value.announcement.trim().is_empty() {
            return Err("Annoucement cannot be empty".to_string());
        }

        Ok(Self {
            id: Uuid::new_v4(),
            announcement: value.announcement.clone(),
            announcer_id: value.announcer_id,
            class_id: value.class_id,
        })
    }
}

pub struct AnnouncerName(String);

impl AnnouncerName {
    pub fn parse(s: String) -> Result<AnnouncerName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.len() > 256;

        if is_empty_or_whitespace || is_too_long {
            Err(format!("{} is not a valid name", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for AnnouncerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::AnnouncerName;

    #[test]
    fn a_256_length_name_is_valid() {
        let name = "a".repeat(256);
        assert!(AnnouncerName::parse(name).is_ok())
    }
    #[test]
    fn a_name_longer_than_256_is_invalid() {
        let name = "a".repeat(257);
        assert!(AnnouncerName::parse(name).is_err())
    }
    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert!(AnnouncerName::parse(name).is_err())
    }
    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert!(AnnouncerName::parse(name).is_err())
    }
    #[test]
    fn valid_name_is_parsed_successfully() {
        let name = "Suleman Mahmood".to_string();
        assert!(AnnouncerName::parse(name).is_ok())
    }
}
