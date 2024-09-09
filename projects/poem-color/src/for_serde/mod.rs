use crate::{ApiError, PoemResult};
use serde::{Serialize, Serializer, ser::SerializeStruct};

impl<T, E: ApiError> Serialize for PoemResult<T, E>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = Serializer::serialize_struct(serializer, "ApiResult", 2)?;
        match self {
            PoemResult::Success(value) => {
                ser.serialize_field("code", &0)?;
                ser.serialize_field("data", value)?;
            }
            PoemResult::Failure(error) => {
                ser.serialize_field("code", &error.error_code())?;
                ser.serialize_field("message", &error.error_message())?;
            }
        }
        ser.end()
    }
}
