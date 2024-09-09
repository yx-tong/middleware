use crate::PoemResult;
use utoipa::{
    ToSchema,
    openapi::{
        KnownFormat, OneOfBuilder, RefOr, Schema, SchemaFormat,
        schema::{ObjectBuilder, SchemaType, Type},
    },
};

impl<'a, T, E> ToSchema<'a> for PoemResult<T, E> {
    fn schema() -> (&'a str, RefOr<Schema>) {
        let success = ObjectBuilder::new()
            .property("code", i32_schema())
            .property("data", utoipa::openapi::schema::RefBuilder::new().ref_location_from_schema_name("T"))
            .required("code")
            .required("data");
        let failure = ObjectBuilder::new()
            .property("code", i32_schema())
            .property("message", ObjectBuilder::new().schema_type(SchemaType::new(Type::String)))
            .property("trace", ObjectBuilder::new().schema_type(SchemaType::new(Type::String)))
            .required("code")
            .required("message");
        ("ApiResult", OneOfBuilder::new().item(success).item(failure).into())
    }
    fn aliases() -> Vec<(&'a str, Schema)> {
        Vec::new()
    }
}

fn i32_schema() -> ObjectBuilder {
    ObjectBuilder::new().schema_type(SchemaType::new(Type::Integer)).format(Some(SchemaFormat::KnownFormat(KnownFormat::Int32)))
}
