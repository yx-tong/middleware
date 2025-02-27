use crate::{YxError, YxErrorKind};
use aliyun_sdk::AliError;

impl From<AliError> for YxError {
    fn from(error: AliError) -> Self {
        YxErrorKind::ServiceError { message: error.to_string() }.into()
    }
}
