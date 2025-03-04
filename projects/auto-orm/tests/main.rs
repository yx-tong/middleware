pub mod examples;
#[test]
fn ready() {
    println!("it works!")
}

pub struct JsonUrl<T>(pub T);

impl<T: ParseFromJSON> ParsePayload for JsonUrl<T> {
    const IS_REQUIRED: bool = false;

    async fn from_request(request: &Request, body: &mut RequestBody) -> poem::Result<Self> {
        let content_type = request
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|content_type| content_type.to_str().ok())
            .map(|x| x.contains("json"))
            .unwrap_or_default();
        if content_type {
            let data = &body.take()?.into_bytes().await?;
            let json = match serde_json::from_slice(data) {
                Ok(o) => Ok(o),
                Err(e) => Err(ParseJsonError::InvalidContentType("T4".to_string())),
            }?;
            match T::parse_from_json(Some(json)) {
                Ok(o) => Ok(JsonUrl(o)),
                Err(e) => Err(ParseJsonError::InvalidContentType("T2".to_string()).into()),
            }
        }
        else {
            let out = request
                .uri()
                .query()
                .map(|s| s.trim_start_matches("json="))
                .and_then(|s| urlencoding::decode(s).ok())
                .and_then(|s| T::parse_from_json_string(&s).ok())
                .ok_or(ParseJsonError::InvalidContentType("T3".to_string()))?;
            Ok(JsonUrl(out))
        }
    }
}

impl<T: poem_openapi::types::Type + Send> poem_openapi::payload::Payload for JsonUrl<T> {
    const CONTENT_TYPE: &'static str = "application/json; charset=utf-8";

    fn check_content_type(content_type: &str) -> bool {
        content_type.starts_with("application") && content_type.contains("json")
    }

    fn schema_ref() -> MetaSchemaRef {
        T::schema_ref()
    }

    #[allow(unused_variables)]
    fn register(registry: &mut Registry) {
        T::register(registry);
    }
}

impl<'a, T: ParseFromJSON> ApiExtractor<'a> for JsonUrl<T> {
    const TYPES: &'static [ApiExtractorType] = &[ApiExtractorType::RequestObject];
    type ParamType = ();
    type ParamRawType = ();

    fn register(registry: &mut ::poem_openapi::registry::Registry) {
        <Self as ::poem_openapi::payload::Payload>::register(registry)
    }
    fn request_meta() -> Option<::poem_openapi::registry::MetaRequest> {
        Some(::poem_openapi::registry::MetaRequest {
            description: None,
            content: vec![poem_openapi::registry::MetaMediaType {
                content_type: <Self as ::poem_openapi::payload::Payload>::CONTENT_TYPE,
                schema: <Self as ::poem_openapi::payload::Payload>::schema_ref(),
            }],
            required: <Self as ::poem_openapi::payload::ParsePayload>::IS_REQUIRED,
        })
    }
    async fn from_request(
        request: &'a Request,
        body: &mut RequestBody,
        _: ExtractParamOptions<Self::ParamType>,
    ) -> poem::Result<Self> {
        <JsonUrl<_> as ParsePayload>::from_request(request, body).await
    }
}
