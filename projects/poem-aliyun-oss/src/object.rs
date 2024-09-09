use crate::auth::Auth;
use reqwest::{
    Method,
    header::{DATE, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::oss::AliyunOssBucket;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommonPrefix {
    prefix: String,
}

impl CommonPrefix {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListObjects {
    name: String,
    delimiter: String,
    prefix: String,
    marker: String,
    max_keys: String,
    is_truncated: bool,

    #[serde(default)]
    contents: Vec<Object>,
    #[serde(default)]
    common_prefixes: Vec<CommonPrefix>,
}

impl ListObjects {
    pub fn new(
        name: String,
        delimiter: String,
        prefix: String,
        marker: String,
        max_keys: String,
        is_truncated: bool,

        contents: Vec<Object>,
        common_prefixes: Vec<CommonPrefix>,
    ) -> Self {
        ListObjects { name, delimiter, prefix, marker, max_keys, is_truncated, contents, common_prefixes }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn delimiter(&self) -> &str {
        &self.delimiter
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn marker(&self) -> &str {
        &self.marker
    }

    pub fn max_keys(&self) -> &str {
        &self.max_keys
    }

    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }

    pub fn contents(&self) -> &Vec<Object> {
        &self.contents
    }

    pub fn common_prefixes(&self) -> &Vec<CommonPrefix> {
        &self.common_prefixes
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Owner {
    #[serde(alias = "ID")]
    pub id: String,
    pub display_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Object {
    key: String,
    last_modified: String,
    size: usize,
    e_tag: String,
    r#type: String,
    storage_class: String,
    owner: Owner,
}

impl Object {
    pub fn new(
        key: String,
        last_modified: String,
        size: usize,

        e_tag: String,
        r#type: String,
        storage_class: String,
        owner: Owner,
    ) -> Self {
        Object { key, last_modified, size, e_tag, r#type, storage_class, owner }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn last_modified(&self) -> &str {
        &self.last_modified
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn e_tag(&self) -> &str {
        &self.e_tag
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    pub fn storage_class(&self) -> &str {
        &self.storage_class
    }

    pub fn id(&self) -> &str {
        &self.owner.id
    }

    pub fn display_name(&self) -> &str {
        &self.owner.display_name
    }
}

impl AliyunOssBucket {
    pub fn sign_path(&self, method: Method, object_name: &str, expires: u64) -> HashMap<String, String> {
        let object_name = object_name.as_ref();
        let mut headers = HeaderMap::new();
        headers.insert(DATE, HeaderValue::from_str(&expires.to_string()).unwrap());
        let signature = self.sign(method.as_str(), self.key_secret(), self.bucket(), object_name, "", &headers);
        let mut sign = HashMap::default();
        sign.insert("Expires".to_string(), expires.to_string());
        sign.insert("OSSAccessKeyId".to_string(), self.key_id().to_string());
        sign.insert("Signature".to_string(), signature);
        sign
    }
}
