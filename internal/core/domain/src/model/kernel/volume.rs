use crate::errors::domain_model_errors::DomainModelError;

#[derive(Clone, Copy, Debug)]
pub struct Volume(u16);

impl Volume {
    pub fn new(v: u16) -> Result<Self, DomainModelError> {
        if v == 0 {
            return Err(DomainModelError::ArgumentCannotBeZero("volume".to_string()));
        }

        Ok(Self(v))
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}
