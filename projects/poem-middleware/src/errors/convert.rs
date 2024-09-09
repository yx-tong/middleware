use sea_orm::DbErr;
use super::*;

impl From<YxErrorKind> for YxError {
    fn from(value: YxErrorKind) -> Self {
        Self {
            kind: Box::new(value),
        }
    }
}

impl From<DbErr> for YxError {
    fn from(value: DbErr) -> Self {
        Self {
            kind: Box::new(YxErrorKind::DatabaseError {
                message: value.to_string(),
            }),
        }
    }
}


impl From<reqwest::Error> for YxError {
    fn from(value: reqwest::Error) -> Self {
        Self {
            kind: Box::new(YxErrorKind::DatabaseError {
                message: value.to_string(),
            }),
        }
    }
}