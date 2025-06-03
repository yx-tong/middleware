use poem_openapi::{
    __private::serde_json::Map,
    Object,
    registry::{MetaSchema, MetaSchemaRef, Registry},
    types::{IsObjectType, ParseError, ParseFromJSON, ParseFromXML, ToJSON, Type},
};
use serde_json::Value;
use std::{any::type_name, borrow::Cow};

#[derive(Copy, Clone, Debug)]
pub struct Pager<const DEFAULT: u32, const LIMIT: u32> {
    /// The current page number
    page: u32,
    /// The number of items per page
    size: u32,
}

impl<const DEFAULT: u32, const LIMIT: u32> Type for Pager<DEFAULT, LIMIT> {
    const IS_REQUIRED: bool = false;
    type RawValueType = Self;
    type RawElementValueType = Self;
    fn name() -> Cow<'static, str> {
        Cow::Owned(format!("Pager<{}, {}>", DEFAULT, LIMIT))
    }
    fn schema_ref() -> MetaSchemaRef {
        let meta = MetaSchema {
            description: None,
            external_docs: None,
            required: vec!["page", "size"],
            properties: {
                let mut fields = Vec::new();
                {
                    let original_schema = u32::schema_ref();
                    let patch_schema = {
                        let mut schema = MetaSchema::ANY;
                        schema.default = Some(Value::from(0));
                        schema.read_only = true;
                        schema.write_only = false;
                        schema.description = Some("The current page number");
                        schema
                    };
                    fields.push(("page", original_schema.merge(patch_schema)));
                }
                {
                    let original_schema = <Option<u32> as Type>::schema_ref();
                    let patch_schema = {
                        let mut schema = MetaSchema::ANY;
                        schema.default = Some(Value::from(DEFAULT));
                        schema.read_only = true;
                        schema.write_only = false;
                        schema.description = Some("The number of items per page");
                        schema
                    };
                    fields.push(("size", original_schema.merge(patch_schema)));
                }
                fields
            },
            deprecated: false,
            ..MetaSchema::new("object")
        };
        MetaSchemaRef::Inline(Box::new(meta))
    }
    fn register(_: &mut Registry) {
        // Virtual types do not require registration
    }
    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }
    fn raw_element_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.as_raw_value().into_iter())
    }
}
impl<const DEFAULT: u32, const LIMIT: u32> IsObjectType for Pager<DEFAULT, LIMIT> {}
impl<const DEFAULT: u32, const LIMIT: u32> ParseFromJSON for Pager<DEFAULT, LIMIT> {
    fn parse_from_json(value: Option<Value>) -> Result<Self, ParseError<Self>> {
        let value = value.unwrap_or_default();
        match value {
            Value::Object(mut obj) => {
                let page = u32::parse_from_json(obj.remove("page")).unwrap_or(0);
                let size = u32::parse_from_json(obj.remove("size")).unwrap_or(DEFAULT);
                Ok(Self { page, size })
            }
            Value::Number(s) => match s.as_u64() {
                Some(s) => Ok(Self { page: s as u32, size: DEFAULT }),
                None => Err(ParseError::custom("must number")),
            },
            Value::Null => Ok(Self { page: 0, size: DEFAULT }),
            _ => Err(ParseError::expected_type(value)),
        }
    }
}
impl<const DEFAULT: u32, const LIMIT: u32> ToJSON for Pager<DEFAULT, LIMIT> {
    fn to_json(&self) -> Option<Value> {
        let mut object = Map::new();
        object.insert("page".to_string(), self.page.into());
        object.insert("size".to_string(), self.size.into());
        Some(Value::Object(object))
    }
}

impl<const DEFAULT: u32, const LIMIT: u32> ParseFromXML for Pager<DEFAULT, LIMIT> {
    fn parse_from_xml(value: Option<Value>) -> Result<Self, ParseError<Self>> {
        Self::parse_from_json(value)
    }
}
impl<const DEFAULT: u32, const LIMIT: u32> poem_openapi::types::ToXML for Pager<DEFAULT, LIMIT> {
    fn to_xml(&self) -> Option<Value> {
        self.to_json()
    }
}
impl<const DEFAULT: u32, const LIMIT: u32> poem_openapi::types::ParseFromYAML for Pager<DEFAULT, LIMIT> {
    fn parse_from_yaml(value: Option<Value>) -> Result<Self, ParseError<Self>> {
        Self::parse_from_json(value)
    }
}
impl<const DEFAULT: u32, const LIMIT: u32> poem_openapi::types::ToYAML for Pager<DEFAULT, LIMIT> {
    fn to_yaml(&self) -> Option<Value> {
        self.to_json()
    }
}

impl<const DEFAULT: u32, const LIMIT: u32> Pager<{ DEFAULT }, { LIMIT }> {
    pub fn offset(&self) -> u64 {
        let page = self.page as u64;
        let size = self.size as u64;
        page * size
    }
    pub fn limit(&self) -> u64 {
        self.size.min(LIMIT) as u64
    }
    pub fn count(&self, count: u64) -> PageCounter {
        let pages = (count as f32 / self.size as f32).ceil() as u32;
        PageCounter { pages, total: count }
    }
}

#[derive(Object)]
pub struct PageCounter {
    pub total: u64,
    pub pages: u32,
}
// #[derive(Object)]
pub struct CountableList<T> {
    pub items: Vec<T>,
    // #[oai(flatten)]
    pub count: PageCounter,
}
impl<T: Type> Type for CountableList<T> {
    const IS_REQUIRED: bool = true;
    type RawValueType = Self;
    type RawElementValueType = Self;
    fn name() -> Cow<'static, str> {
        Cow::Owned(format!("CountableList<{}>", type_name::<T>()))
    }
    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Reference(<Self as Type>::name().into_owned())
    }
    fn register(registry: &mut Registry) {
        registry.create_schema::<Self, _>(<Self as Type>::name().into_owned(), |registry| {
            <Vec<T> as Type>::register(registry);
            let mut meta = MetaSchema {
                description: None,
                external_docs: None,
                required: vec!["items", "pages", "total"],
                properties: {
                    let mut fields = Vec::new();
                    {
                        let original_schema = <Vec<T> as Type>::schema_ref();
                        let patch_schema = {
                            let mut schema = MetaSchema::ANY;
                            schema.default = None;
                            schema.read_only = false;
                            schema.write_only = false;
                            if let Some(field_description) = None {
                                schema.description = Some(field_description);
                            }
                            schema
                        };
                        fields.push(("items", original_schema.merge(patch_schema)));
                    }
                    fields.extend(registry.create_fake_schema::<PageCounter>().properties);
                    fields
                },
                deprecated: false,
                ..MetaSchema::new("object")
            };
            meta.example = None;
            meta
        })
    }
    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }
    fn raw_element_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(::std::iter::IntoIterator::into_iter(self.as_raw_value()))
    }
}
impl<T: Type + Send + Sync + ParseFromJSON + ToJSON> IsObjectType for CountableList<T> {}
impl<T: Type + Send + Sync + ParseFromJSON + ToJSON> ParseFromJSON for CountableList<T> {
    fn parse_from_json(value: Option<Value>) -> Result<Self, ParseError<Self>> {
        let value = value.unwrap_or_default();
        match value {
            Value::Object(mut obj) => {
                let items: Vec<T> = {
                    let value = ParseFromJSON::parse_from_json(obj.remove("items")).map_err(ParseError::propagate)?;
                    value
                };
                let count: PageCounter =
                    { ParseFromJSON::parse_from_json(Some(Value::Object(obj))).map_err(ParseError::propagate)? };
                Ok(Self { items, count })
            }
            _ => Err(ParseError::expected_type(value)),
        }
    }
}
impl<T: Type + Send + Sync + ParseFromJSON + ToJSON> ToJSON for CountableList<T> {
    fn to_json(&self) -> Option<Value> {
        let mut object = Map::new();
        object.insert("items".to_string(), self.items.to_json()?);
        object.insert("pages".to_string(), self.count.pages.to_json()?);
        object.insert("total".to_string(), self.count.total.to_json()?);
        Some(Value::Object(object))
    }
}
impl<T: Type + Send + Sync + ParseFromJSON + ToJSON> ParseFromXML for CountableList<T> {
    fn parse_from_xml(value: Option<Value>) -> Result<Self, ParseError<Self>> {
        Self::parse_from_json(value)
    }
}
impl<T: Type + Send + Sync + ParseFromJSON + ToJSON> poem_openapi::types::ToXML for CountableList<T> {
    fn to_xml(&self) -> Option<Value> {
        self.to_json()
    }
}
impl<T: Type + Send + Sync + ParseFromJSON + ToJSON> poem_openapi::types::ParseFromYAML for CountableList<T> {
    fn parse_from_yaml(value: Option<Value>) -> Result<Self, ParseError<Self>> {
        Self::parse_from_json(value)
    }
}
impl<T: Type + Send + Sync + ParseFromJSON + ToJSON> poem_openapi::types::ToYAML for CountableList<T> {
    fn to_yaml(&self) -> Option<Value> {
        self.to_json()
    }
}
