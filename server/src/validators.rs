use validator::ValidationError;

pub fn is_valid_slug(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

pub fn slug_validator(s: &str) -> Result<(), ValidationError> {
    if is_valid_slug(s) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_format"))
    }
}
