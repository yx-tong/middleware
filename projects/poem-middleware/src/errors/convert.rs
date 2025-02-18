use super::*;
// use pdf_extract::OutputError;

impl From<YxErrorKind> for YxError {
    fn from(value: YxErrorKind) -> Self {
        Self { kind: Box::new(value) }
    }
}

// impl From<DbErr> for YxError {
//     fn from(value: DbErr) -> Self {
//         Self { kind: Box::new(YxErrorKind::DatabaseError { message: value.to_string() }) }
//     }
// }
impl From<sqlx::Error> for YxError {
    fn from(value: sqlx::Error) -> Self {
        Self { kind: Box::new(YxErrorKind::DatabaseError { message: value.to_string() }) }
    }
}

impl From<reqwest::Error> for YxError {
    fn from(value: reqwest::Error) -> Self {
        Self { kind: Box::new(YxErrorKind::ServiceError { message: value.to_string() }) }
    }
}

impl From<poem::Error> for YxError {
    fn from(value: poem::Error) -> Self {
        Self { kind: Box::new(YxErrorKind::ServiceError { message: value.to_string() }) }
    }
}

impl From<std::io::Error> for YxError {
    fn from(value: std::io::Error) -> Self {
        Self { kind: Box::new(YxErrorKind::ServiceError { message: value.to_string() }) }
    }
}

// impl From<OutputError> for YxError {
//     fn from(value: OutputError) -> Self {
//         Self { kind: Box::new(YxErrorKind::ServiceError { message: value.to_string() }) }
//     }
// }
impl From<serde_json::Error> for YxError {
    fn from(value: serde_json::Error) -> Self {
        Self { kind: Box::new(YxErrorKind::ServiceError { message: value.to_string() }) }
    }
}
// impl From<DashError> for YxError {
//     fn from(value: DashError) -> Self {
//         Self { kind: Box::new(YxErrorKind::ServiceError { message: value.to_string() }) }
//     }
// }
