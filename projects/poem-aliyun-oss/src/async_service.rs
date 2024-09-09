use super::{auth::*, errors::OssError, oss::AliyunOssBucket};
use crate::service::ListBuckets;
use reqwest::header::{DATE, HeaderMap};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Bucket {
    name: String,
    creation_date: String,
    location: String,
    extranet_endpoint: String,
    intranet_endpoint: String,
    storage_class: String,
}

impl Bucket {
    pub fn new(
        name: String,
        creation_date: String,
        location: String,
        extranet_endpoint: String,
        intranet_endpoint: String,
        storage_class: String,
    ) -> Self {
        Bucket { name, creation_date, location, extranet_endpoint, intranet_endpoint, storage_class }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn creation_date(&self) -> &str {
        &self.creation_date
    }

    pub fn location(&self) -> &str {
        &self.location
    }

    pub fn extranet_endpoint(&self) -> &str {
        &self.extranet_endpoint
    }

    pub fn intranet_endpoint(&self) -> &str {
        &self.intranet_endpoint
    }

    pub fn storage_class(&self) -> &str {
        &self.storage_class
    }
}

impl AliyunOssBucket {
    pub async fn list_bucket(&self, resources: Option<HashMap<String, String>>) -> Result<ListBuckets, OssError> {
        let resources_str = if let Some(r) = resources { self.get_resources_str(&r) } else { String::new() };
        // let host = self.endpoint();
        let date = self.date();

        let mut headers = HeaderMap::new();
        headers.insert(DATE, date.parse()?);
        let authorization = self.oss_sign("GET", self.key_id(), self.key_secret(), "", "", &resources_str, &headers);
        headers.insert("Authorization", authorization.parse()?);

        let resp = self.http_client.get(self.bucket()).headers(headers).send().await?;

        let body = resp.text().await?;
        let list_buckets = quick_xml::de::from_str::<ListBuckets>(&body)?;

        Ok(list_buckets)
    }
}
