use crate::domain::user::error::UserDomainError;

pub struct Email(String);
pub struct Username(String);

impl Email {
    pub fn new(value: String) -> Result<Self, UserDomainError> {
        if value.trim().is_empty() {
            return Err(UserDomainError::InvalidEmail(
                "cannot be empty.".to_string(),
            ));
        }

        if !value.contains('@') {
            return Err(UserDomainError::InvalidEmail("must contains @".to_string()));
        }

        if value.len() > 255 {
            return Err(UserDomainError::InvalidEmail("too long.".to_string()));
        }

        Ok(Self(value))
    }
}

impl Username {
    pub fn new(value: String) -> Result<Self, UserDomainError> {
        if value.len() < 3 {
            return Err(UserDomainError::InvalidUsername("too short.".to_string()));
        }

        if value.len() > 255 {
            return Err(UserDomainError::InvalidUsername("too long.".to_string()));
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
