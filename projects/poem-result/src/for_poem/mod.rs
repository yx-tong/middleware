use crate::{ApiError, PoemResult};
use poem_openapi::{
    __private::poem::{IntoResponse, Response},
    ApiResponse,
    registry::{MetaMediaType, MetaResponse, MetaResponses, MetaSchema, MetaSchemaRef, Registry},
    types::{IsObjectType, ParseError, ParseFromJSON, ToJSON, Type},
};
use serde_json::{Number, Value};
use std::{any::type_name, borrow::Cow};

impl<T: Type, E: ApiError + Send + Sync> ApiResponse for PoemResult<T, E> {
    const BAD_REQUEST_HANDLER: bool = false;

    fn meta() -> MetaResponses {
        MetaResponses {
            responses: vec![MetaResponse {
                description: "service ready",
                status: Some(200),
                status_range: None,
                content: vec![MetaMediaType {
                    content_type: "application/json; charset=utf-8",
                    schema: <Self as Type>::schema_ref(),
                }],
                headers: vec![],
            }],
        }
    }

    fn register(registry: &mut Registry) {
        <Self as Type>::register(registry);
    }
}

impl<T: ToJSON + Send + Type, E: ApiError + Send + Sync> IntoResponse for PoemResult<T, E> {
    fn into_response(self) -> Response {
        Response::builder().body(self.to_json_string()).into_response()
    }
}

impl<T: Type, E: ApiError + Send + Sync> Type for PoemResult<T, E> {
    const IS_REQUIRED: bool = true;
    type RawValueType = Self;
    type RawElementValueType = Self;
    fn name() -> Cow<'static, str> {
        let mut name = ::std::string::String::from("ApiResult");
        name.push('<');
        name.push_str(&type_name::<T>());
        name.push_str(", ");
        name.push_str(&type_name::<E>());
        name.push('>');
        Cow::Owned(name)
    }
    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Reference(Self::name().into_owned())
    }
    fn register(registry: &mut Registry) {
        let mut required = Vec::new();
        required.push("code");
        required.push("data");
        registry.create_schema::<Self, _>(<Self as Type>::name().into_owned(), |registry| {
            <T as Type>::register(registry);
            let mut meta = MetaSchema {
                description: None,
                external_docs: None,
                required,
                properties: {
                    let mut fields = Vec::new();
                    {
                        let original_schema = <i32 as Type>::schema_ref();
                        let patch_schema = {
                            let mut schema = MetaSchema::ANY;
                            schema.default = None;
                            schema.read_only = true;
                            schema.write_only = false;
                            if let Some(field_description) = None {
                                schema.description = Some(field_description);
                            }
                            schema
                        };
                        fields.push(("code", original_schema.merge(patch_schema)));
                    }
                    {
                        let original_schema = <String as Type>::schema_ref();
                        let patch_schema = {
                            let mut schema = MetaSchema::ANY;
                            schema.ty = "string";
                            schema.default = None;
                            schema.read_only = true;
                            schema.write_only = false;
                            if let Some(field_description) = None {
                                schema.description = Some(field_description);
                            }
                            schema
                        };
                        fields.push(("message", original_schema.merge(patch_schema)));
                    }
                    {
                        let original_schema = <T as Type>::schema_ref();
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
                        fields.push(("data", original_schema.merge(patch_schema)));
                    }
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

impl<T: Type, E: ApiError + Send + Sync> IsObjectType for PoemResult<T, E> {}

impl<T: Type + ParseFromJSON, E: ApiError + Send + Sync> ParseFromJSON for PoemResult<T, E> {
    fn parse_from_json(value: Option<Value>) -> Result<Self, ParseError<Self>> {
        match value {
            Some(Value::Object(mut map)) => match ParseFromJSON::parse_from_json(map.remove("data")) {
                Ok(result) => Ok(PoemResult::Success(result)),
                Err(e) => Err(ParseError::propagate(e)),
            },
            Some(ty) => Err(ParseError::expected_type(ty)),
            None => Err(ParseError::expected_type(Value::Null)),
        }
    }
}
impl<T: Type + ToJSON, E: ApiError + Send + Sync> ToJSON for PoemResult<T, E> {
    fn to_json(&self) -> Option<Value> {
        let mut object = poem_openapi::__private::serde_json::Map::new();
        match self {
            PoemResult::Success(s) => {
                object.insert("code".to_string(), Value::Number(Number::from(0)));
                match ToJSON::to_json(s) {
                    Some(s) => object.insert("data".to_string(), s),
                    None => {
                        object.insert("code".to_string(), Value::Number(Number::from(-1)));
                        object.insert("message".to_string(), Value::String("write data failed".to_string()))
                    }
                };
            }
            PoemResult::Failure(e) => {
                object.insert("code".to_string(), Value::Number(Number::from(e.error_code())));
                object.insert("message".to_string(), Value::String(e.error_message().to_string()));
            }
        }
        Some(Value::Object(object))
    }
}
