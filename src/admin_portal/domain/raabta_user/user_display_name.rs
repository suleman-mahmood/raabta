use regex::Regex;

pub struct DisplayName(String);
impl DisplayName {
    pub fn derive_from_student(display_name: &Self) -> Self {
        Self(format!("{}'s Parent", display_name.0))
    }

    pub fn parse(display_name: &str) -> Result<Self, String> {
        let display_name = display_name.trim();
        let display_name_regex_result = Regex::new(r#"^[\d '\w]{3,50}$"#);
        match display_name_regex_result {
            Ok(display_name_regex) => match display_name_regex.is_match(display_name) {
                true => Ok(Self(display_name.to_string())),
                false => Err(format!(
                    "Display name regex doesn't match for value: {}",
                    display_name
                )),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
impl AsRef<str> for DisplayName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
