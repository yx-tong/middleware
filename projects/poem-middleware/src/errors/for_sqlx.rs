use crate::{YxError, YxErrorKind};
use sqlx::Error;

impl From<Error> for YxError {
    fn from(value: Error) -> Self {
        let kind = match &value {
            Error::Database(e) => YxErrorKind::DatabaseError { message: e.to_string() },
            _ => YxErrorKind::DatabaseError { message: value.to_string() },
        };
        Self { kind: Box::new(kind) }
    }
}
