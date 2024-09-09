use super::*;

use poem_openapi::{
    __private::serde_json::Value,
    registry::{MetaSchema, MetaSchemaRef, Registry},
    types::{IsObjectType, ParseError, ParseFromJSON, ToJSON, Type},
};
use std::borrow::Cow;

impl<T: Type + Send + Sync + ParseFromJSON + ToJSON> Type for ItemList<T> {
    const IS_REQUIRED: bool = false;
    type RawValueType = Self;
    type RawElementValueType = Self;
    fn name() -> Cow<'static, str> {
        Cow::Owned(format!("ItemList<{}>", T::name()))
    }
    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Box::new(MetaSchema { items: Some(Box::new(T::schema_ref())), ..MetaSchema::new("array") }))
    }
    fn register(registry: &mut Registry) {
        registry.create_schema::<Self, _>(Self::name().into_owned(), |registry| {
            T::register(registry);
            Vec::<T>::register(registry);
            MetaSchema {
                ty: "object",
                // format: Some("array"),
                title: Some(format!("ItemList<{}>", T::name())),
                description: Some("ItemList description"),
                external_docs: None,
                one_of: vec![T::schema_ref(), Vec::<T>::schema_ref()],
                any_of: Vec::new(),
                items: Some(Box::new(T::schema_ref())),
                discriminator: None,
                ..MetaSchema::ANY
            }
        });
    }
    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }
    fn raw_element_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(::std::iter::IntoIterator::into_iter(self.as_raw_value()))
    }
}
impl<T: Type + Send + Sync + ParseFromJSON + ToJSON> IsObjectType for ItemList<T> {}

impl<T: Type + Send + Sync + ParseFromJSON + ToJSON> ParseFromJSON for ItemList<T> {
    fn parse_from_json(value: Option<Value>) -> Result<Self, ParseError<Self>> {
        let items = match value {
            None => Vec::new(),
            Some(Value::Array(v)) => {
                let mut items = Vec::with_capacity(v.len());
                for item in v {
                    match T::parse_from_json(Some(item)) {
                        Ok(o) => items.push(o),
                        Err(e) => return Err(ParseError::custom(e.message())),
                    }
                }
                items
            }
            Some(v) => match T::parse_from_json(Some(v)) {
                Ok(o) => {
                    vec![o]
                }
                Err(e) => return Err(ParseError::custom(e.message())),
            },
        };
        Ok(Self { list: items })
    }
}

impl<T: Type + Send + Sync + ParseFromJSON + ToJSON> ToJSON for ItemList<T> {
    fn to_json(&self) -> Option<Value> {
        self.list.to_json()
    }
}
