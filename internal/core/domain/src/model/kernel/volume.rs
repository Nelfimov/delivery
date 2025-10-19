use crate::errors::domain_model_errors::DomainModelError;

pub struct Volume(pub u16);

impl Volume {
    pub fn new(v: u16) -> Result<Self, DomainModelError> {
        if v == 0 {
            return Err(DomainModelError::ArgumentCannotBeZero("volume".to_string()));
        }

        Ok(Self(v))
    }
}
