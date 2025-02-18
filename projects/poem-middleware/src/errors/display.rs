use super::*;

impl Error for YxError {}

impl Debug for YxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for YxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for YxErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            YxErrorKind::UnknownError => {
                write!(f, "UnknownError")
            }
            YxErrorKind::DatabaseError { message } => {
                write!(f, "{}", message)
            }
            YxErrorKind::ServiceError { message } => {
                write!(f, "{}", message)
            }
        }
    }
}
