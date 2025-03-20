use regex::Regex;

#[derive(Clone)]
pub struct UserPhoneNumber(Option<String>);
impl UserPhoneNumber {
    pub fn parse(phone_number: String) -> UserPhoneNumber {
        let phone_number = phone_number.trim();
        let phone_regex_result = Regex::new(r"^\d{4}-\d{7}$");
        match phone_number.is_empty() {
            true => Self(None),
            false => match phone_regex_result {
                Ok(phone_regex) => {
                    if phone_regex.is_match(phone_number) {
                        Self(Some(phone_number.to_string()))
                    } else {
                        Self(None)
                    }
                }
                Err(_) => Self(None),
            },
        }
    }

    pub fn parse_with_error(phone_number: String) -> Result<UserPhoneNumber, String> {
        let phone_number = phone_number.trim();
        let phone_regex_result = Regex::new(r"^\d{4}-\d{7}$");
        match phone_number.is_empty() {
            true => Ok(Self(None)),
            false => match phone_regex_result {
                Ok(phone_regex) => {
                    if phone_regex.is_match(phone_number) {
                        Ok(Self(Some(phone_number.to_string())))
                    } else {
                        Err(format!(
                            "Phone number {:?} doesn't pass regex check",
                            phone_number
                        ))
                    }
                }
                Err(e) => Err(e.to_string()),
            },
        }
    }
}
impl AsRef<Option<String>> for UserPhoneNumber {
    fn as_ref(&self) -> &Option<String> {
        &self.0
    }
}
