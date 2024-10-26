use regex::Regex;

use super::DisplayName;

pub struct UserEmail(String);
impl UserEmail {
    pub fn derive_from_student(email: &Self) -> Self {
        let mut slice_iter = email.0.split("@");
        let first_part = slice_iter.next().unwrap();
        let second_part = slice_iter.next().unwrap();
        Self(format!("{}.parent@{}", first_part, second_part))
    }

    pub fn derive_from_display_name(display_name: &DisplayName) -> Self {
        let first_word = display_name
            .as_ref()
            .split(" ")
            .next()
            .unwrap()
            .to_lowercase();
        Self(format!("{}@riveroaks.com", first_word))
    }

    pub fn parse(email: String) -> Result<Self, String> {
        let email = email.trim();
        let email_regex_result = Regex::new(
            r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
        );

        match email.is_empty() {
            true => Err(format!("Email passed is empty: {:?}", email)),
            false => match email_regex_result {
                Ok(email_regex) => {
                    if email_regex.is_match(email) {
                        Ok(Self(email.to_string()))
                    } else {
                        Err(format!("Email passed doesn't match regex: {:?}", email))
                    }
                }
                Err(e) => Err(e.to_string()),
            },
        }
    }
}
impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl Default for UserEmail {
    fn default() -> Self {
        Self("random.user@raabta.com".to_string())
    }
}
