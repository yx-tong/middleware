use super::*;
use poem_email::AddressError;

impl From<poem_email::Error> for YxError {
    fn from(value: poem_email::Error) -> Self {
        Self { kind: Box::new(YxErrorKind::ServiceError { message: value.to_string() }) }
    }
}

impl From<AddressError> for YxError {
    fn from(value: AddressError) -> Self {
        Self { kind: Box::new(YxErrorKind::ServiceError { message: value.to_string() }) }
    }
}
