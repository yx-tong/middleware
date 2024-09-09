use poem::{Endpoint, Middleware, Request};
use std::collections::HashMap;

pub use poem_aliyun_oss::OssError;
use poem_aliyun_oss::{errors::ObjectError, oss::AliyunOssBucket};
use poem_openapi::{Enum, Object};

/// Middleware for [`tracing`](https://crates.io/crates/tracing).
#[derive(Clone)]
pub struct OssMiddleware {
    buckets: HashMap<String, AliyunOssBucket>,
}

impl OssMiddleware {
    pub fn get_bucket(&self, buckets: &str) -> Result<&AliyunOssBucket, OssError> {
        match self.buckets.get(buckets) {
            Some(s) => Ok(s),
            None => Err(OssError::Object(ObjectError::GetError { msg: "".to_string() })),
        }
    }
}

/// Endpoint for the `Tracing` middleware.
pub struct OssEndpoint<E> {
    inner: E,
    buckets: OssMiddleware,
}

#[derive(Debug, Object)]
pub struct OssUploadRequest {
    path: Vec<String>,
    #[oai(default)]
    auth: OssAuthorization,
    time: Option<u64>,
}

#[derive(Debug, Object)]
pub struct OssUploadResponse {
    urls: Vec<String>,
}

#[derive(Debug, Object)]
pub struct OssDownloadRequest {
    path: Vec<String>,
    #[oai(default)]
    auth: OssAuthorization,
    time: Option<u64>,
}

#[derive(Debug, Object)]
pub struct OssDownloadResponse {
    urls: Vec<String>,
}

impl OssDownloadRequest {
    pub fn execute(&self, env: &OssMiddleware, bucket: &str) -> Result<OssDownloadResponse, OssError> {
        let time = self.time.unwrap_or(60);
        let mut urls = Vec::with_capacity(self.path.len());
        for path in self.path.iter() {
            let url = env.get_bucket(bucket)?.get_object_url(&path, time)?;
            urls.push(url)
        }
        Ok(OssDownloadResponse { urls })
    }
}
impl OssUploadRequest {
    pub fn execute(&self, env: &OssMiddleware, bucket: &str) -> Result<OssUploadResponse, OssError> {
        let time = self.time.unwrap_or(60);
        let mut urls = Vec::with_capacity(self.path.len());
        for path in self.path.iter() {
            let url = env.get_bucket(bucket)?.put_object_url(&path, time)?;
            urls.push(url)
        }
        Ok(OssUploadResponse { urls })
    }
}
#[derive(Debug, Default, Enum)]
pub enum OssAuthorization {
    #[default]
    Unlimited,
    UrlV1,
    UrlV4,
    AuthorizationV1,
    AuthorizationV4,
}

impl OssMiddleware {
    pub fn new(key_id: &str, key_secret: &str, endpoint: &str, bucket: &str) -> Self {
        let mut map = HashMap::new();
        map.insert(bucket.to_string(), AliyunOssBucket::new(key_id, key_secret, endpoint, bucket));
        Self { buckets: map }
    }
}

impl<E: Endpoint> Middleware<E> for OssMiddleware {
    type Output = OssEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        OssEndpoint { inner: ep, buckets: self.clone() }
    }
}

impl<E: Endpoint> Endpoint for OssEndpoint<E> {
    type Output = E::Output;
    async fn call(&self, mut input: Request) -> poem::Result<Self::Output> {
        input.extensions_mut().insert(self.buckets.clone());
        self.inner.call(input).await
    }
}
