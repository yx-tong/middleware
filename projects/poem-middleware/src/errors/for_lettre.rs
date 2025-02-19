use super::*;

impl From<poem_email::Error> for YxError {
    fn from(value: poem_email::Error) -> Self {
        Self { kind: Box::new(YxErrorKind::ServiceError { message: value.to_string() }) }
    }
}
