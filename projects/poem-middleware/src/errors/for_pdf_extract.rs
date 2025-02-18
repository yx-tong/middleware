use super::*;
use pdf_extract::OutputError;

impl From<OutputError> for YxError {
    fn from(value: OutputError) -> Self {
        Self { kind: Box::new(YxErrorKind::ServiceError { message: value.to_string() }) }
    }
}
