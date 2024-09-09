use chrono::prelude::*;
use reqwest::{
    Client, Method,
    header::{DATE, HeaderMap},
};

use std::{
    collections::HashMap,
    str,
    time::{Duration, SystemTime},
};

use crate::errors::ObjectError;

use super::{auth::*, errors::OssError};

const RESOURCES: [&str; 50] = [
    "acl",
    "uploads",
    "location",
    "cors",
    "logging",
    "website",
    "referer",
    "lifecycle",
    "delete",
    "append",
    "tagging",
    "objectMeta",
    "uploadId",
    "partNumber",
    "security-token",
    "position",
    "img",
    "style",
    "styleName",
    "replication",
    "replicationProgress",
    "replicationLocation",
    "cname",
    "bucketInfo",
    "comp",
    "qos",
    "live",
    "status",
    "vod",
    "startTime",
    "endTime",
    "symlink",
    "x-oss-process",
    "response-content-type",
    "response-content-language",
    "response-expires",
    "response-cache-control",
    "response-content-disposition",
    "response-content-encoding",
    "udf",
    "udfName",
    "udfImage",
    "udfId",
    "udfImageDesc",
    "udfApplication",
    "comp",
    "udfApplicationLog",
    "restore",
    "callback",
    "callback-var",
];

#[derive(Clone, Debug)]
pub struct AliyunOssBucket {
    key_id: String,
    key_secret: String,
    endpoint: String,
    bucket: String,
    pub(crate) http_client: Client,
}

#[derive(Default)]
pub struct Options {
    pub pool_max_idle_per_host: Option<usize>,
    pub timeout: Option<Duration>,
}

impl AliyunOssBucket {
    pub fn new<S>(key_id: S, key_secret: S, endpoint: S, bucket: S) -> Self
    where
        S: Into<String>,
    {
        Self::new_with_opts(key_id, key_secret, endpoint, bucket, Default::default())
    }

    pub fn new_with_opts<S>(key_id: S, key_secret: S, endpoint: S, bucket: S, opts: Options) -> Self
    where
        S: Into<String>,
    {
        let mut builder = Client::builder();
        if let Some(timeout) = opts.timeout {
            builder = builder.timeout(timeout);
        }
        if let Some(max_per_host) = opts.pool_max_idle_per_host {
            builder = builder.pool_max_idle_per_host(max_per_host);
        }
        let http_client = builder.build().expect("Build http client failed");
        AliyunOssBucket {
            key_id: key_id.into(),
            key_secret: key_secret.into(),
            endpoint: endpoint.into(),
            bucket: bucket.into(),
            http_client,
        }
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    pub fn key_secret(&self) -> &str {
        &self.key_secret
    }

    pub fn set_bucket(&mut self, bucket: impl Into<String>) {
        self.bucket = bucket.into()
    }

    pub fn host(&self, object: &str, resources_str: &str) -> String {
        format!("https://{}.{}/{}?{}", self.bucket, self.endpoint, object, resources_str)
    }

    pub fn date(&self) -> String {
        let now: DateTime<Utc> = Utc::now();
        now.format("%a, %d %b %Y %T GMT").to_string()
    }

    pub fn get_resources_str(&self, params: &HashMap<String, String>) -> String {
        let mut resources: Vec<(&String, &String)> = params.iter().filter(|(k, _)| RESOURCES.contains(&k.as_ref())).collect();
        resources.sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));
        let mut result = String::new();
        for (k, v) in resources {
            if !result.is_empty() {
                result += "&";
            }
            match v.as_str() {
                "" => {
                    result += k;
                }
                vv => {
                    result += &format!("{}={}", k.to_owned(), vv);
                }
            }
        }
        result
    }

    pub fn get_params_str(&self, params: &HashMap<String, String>) -> String {
        let mut resources: Vec<(&String, &String)> = params.iter().collect();
        resources.sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));
        let mut result = String::new();
        for (k, v) in resources {
            if !result.is_empty() {
                result += "&";
            }
            match v.as_str() {
                "" => {
                    result += k.as_ref();
                }
                vv => {
                    result += &format!("{}={}", k.to_owned(), vv);
                }
            }
        }
        result
    }

    /// Build a request. Return url and header for reqwest client builder.
    pub fn build_request(
        &self,
        req_type: Method,
        object_name: &str,
        mut headers: HeaderMap,
        resources: &HashMap<String, String>,
    ) -> Result<(String, HeaderMap), OssError> {
        let (resources_str, params_str) = (self.get_resources_str(resources), self.get_params_str(resources));

        let host = self.host(object_name, &params_str);
        let date = self.date();
        headers.insert(DATE, date.parse()?);
        let authorization = self.oss_sign(
            req_type.as_str(),
            self.key_id(),
            self.key_secret(),
            self.bucket(),
            object_name,
            &resources_str,
            &headers,
        );
        headers.insert("Authorization", authorization.parse()?);

        Ok((host, headers))
    }
}

#[derive(Debug)]
pub struct ObjectMeta {
    /// The last modified time
    pub last_modified: SystemTime,
    /// The size in bytes of the object
    pub size: usize,
    /// 128-bits RFC 1864 MD5. This field only presents in normal file. Multipart and append-able file will have empty md5.
    pub md5: String,
}

impl ObjectMeta {
    pub fn from_header_map(header: &HeaderMap) -> Result<Self, OssError> {
        let getter = |key: &str| -> Result<&str, OssError> {
            let value = header
                .get(key)
                .ok_or_else(|| {
                    OssError::Object(ObjectError::HeadError {
                        msg: format!("can not find {} in head response, response header: {:?}", key, header).into(),
                    })
                })?
                .to_str()
                .map_err(|_| {
                    OssError::Object(ObjectError::HeadError {
                        msg: format!("header entry {} contains invalid ASCII code", key).into(),
                    })
                })?;
            Ok(value)
        };

        let last_modified = httpdate::parse_http_date(getter("Last-Modified")?).map_err(|e| {
            OssError::Object(ObjectError::HeadError { msg: format!("cannot parse to system time: {}", e).into() })
        })?;
        let size = getter("Content-Length")?
            .parse()
            .map_err(|e| OssError::Object(ObjectError::HeadError { msg: format!("cannot parse to number: {}", e).into() }))?;
        let md5 = getter("Content-Md5")?.to_string();

        Ok(Self { last_modified, size, md5 })
    }
}
