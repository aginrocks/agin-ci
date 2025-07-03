use validator::ValidationError;

static RESERVED_SLUGS: &[&str] = &["new", "edit", "delete", "create"];

pub fn is_valid_slug(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

pub fn slug_validator(s: &str) -> Result<(), ValidationError> {
    if !is_valid_slug(s) {
        return Err(ValidationError::new("invalid_format"));
    }

    if RESERVED_SLUGS.contains(&s) {
        return Err(ValidationError::new("reserved_word"));
    }

    Ok(())
}
