use chrono::NaiveDateTime;
use poem_openapi::{
    __private::serde_json::Number,
    registry::{MetaSchema, MetaSchemaRef, Registry},
    types::{IsObjectType, ParseError, ParseFromJSON, ParseFromXML, ParseFromYAML, ToJSON, ToXML, ToYAML, Type},
};
use serde_json::Value;
use sqlx::{
    self, Database, Decode, Postgres,
    error::BoxDynError,
    postgres::{PgTypeInfo, types::Oid},
    types::chrono::{DateTime, Utc},
};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Copy, Clone, Debug)]
pub struct UnixTime {
    pub time: NaiveDateTime,
}

impl Display for UnixTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.time, f)
    }
}

impl<'de> Decode<'de, Postgres> for UnixTime {
    fn decode(value: <Postgres as Database>::ValueRef<'de>) -> Result<Self, BoxDynError> {
        Ok(Self { time: NaiveDateTime::decode(value)? })
    }
}

impl sqlx::Type<Postgres> for UnixTime {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        PgTypeInfo::with_oid(Oid(1114))
    }
}

impl Type for UnixTime {
    const IS_REQUIRED: bool = true;
    type RawValueType = Self;
    type RawElementValueType = Self;
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("UnixTime")
    }
    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("string", "date-time")))
    }
    fn register(registry: &mut Registry) {
        registry.create_schema::<Self, _>(<Self as Type>::name().into_owned(), |registry| {
            MetaSchema {
                ty: "number",
                title: Some("UnixTime".to_string()),
                format: Some("date-time"),
                description: Some("ISO Time"),
                external_docs: None,
                default: Some(Value::String("1970-01-01T00:00:00.000Z".to_string())),
                // example: Some(Value::String("".to_string())),
                ..MetaSchema::ANY
            }
        })
    }
    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }
    fn raw_element_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.as_raw_value().into_iter())
    }
}
impl IsObjectType for UnixTime {}
impl ParseFromJSON for UnixTime {
    fn parse_from_json(value: Option<Value>) -> Result<Self, ParseError<Self>> {
        let time = value.unwrap_or_default();
        match time {
            Value::Number(unix) => match unix.as_i64().and_then(DateTime::<Utc>::from_timestamp_millis) {
                Some(s) => Ok(Self { time: s.naive_utc() }),
                None => Err(ParseError::custom("Invalid Unix timestamp")),
            },
            Value::String(iso_string) => match DateTime::<Utc>::from_str(&iso_string) {
                Ok(o) => Ok(Self { time: o.naive_utc() }),
                Err(_) => Err(ParseError::custom("Invalid ISO datetime")),
            },
            _ => Err(ParseError::custom("Invalid DateTime")),
        }
    }
}

impl ToJSON for UnixTime {
    fn to_json(&self) -> Option<Value> {
        Some(Value::Number(Number::from(self.time.and_utc().timestamp_millis())))
    }
}
impl ParseFromXML for UnixTime {
    fn parse_from_xml(value: Option<Value>) -> Result<Self, ParseError<Self>> {
        Self::parse_from_json(value)
    }
}

impl ToXML for UnixTime {
    fn to_xml(&self) -> Option<Value> {
        self.to_json()
    }
}
impl ParseFromYAML for UnixTime {
    fn parse_from_yaml(value: Option<Value>) -> Result<Self, ParseError<Self>> {
        Self::parse_from_json(value)
    }
}
impl ToYAML for UnixTime {
    fn to_yaml(&self) -> Option<Value> {
        self.to_json()
    }
}
