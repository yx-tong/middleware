use super::*;
use poem_email::lettre::{address::AddressError, error::Error};

impl From<Error> for YxError {
    fn from(value: Error) -> Self {
        Self { kind: Box::new(YxErrorKind::ServiceError { message: value.to_string() }) }
    }
}

impl From<AddressError> for YxError {
    fn from(value: AddressError) -> Self {
        Self { kind: Box::new(YxErrorKind::DecodeError { format: "email".to_string(), message: value.to_string() }) }
    }
}
