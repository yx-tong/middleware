use crate::{
    multi_part::{CompleteMultipartUploadResult, InitiateMultipartUploadResult},
    oss::ObjectMeta,
};
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use super::errors::{ObjectError, OssError};
use crate::{object::ListObjects, oss::AliyunOssBucket};
use bytes::Bytes;
use reqwest::{Method, header::HeaderMap};

impl AliyunOssBucket {
    pub async fn list_object(&self, headers: HeaderMap, resources: &HashMap<String, String>) -> Result<ListObjects, OssError> {
        let (host, headers) = self.build_request(Method::GET, "", headers, resources)?;
        let resp = self.http_client.get(host).headers(headers).send().await?;
        let body = resp.text().await?;
        let list_objects = quick_xml::de::from_str::<ListObjects>(&body)?;

        Ok(list_objects)
    }

    pub async fn get_object(
        &self,
        object_name: &str,
        headers: HeaderMap,
        resources: &HashMap<String, String>,
    ) -> Result<Bytes, OssError> {
        let (host, headers) = self.build_request(Method::GET, object_name, headers, resources)?;
        println!("HOST: {}", host);

        let resp = self.http_client.get(&host).headers(headers).send().await?;

        if resp.status().is_success() {
            Ok(resp.bytes().await?)
        }
        else {
            Err(OssError::Object(ObjectError::GetError {
                msg: format!("can not get object, status code: {}", resp.status()).into(),
            }))
        }
    }

    pub fn get_object_url(&self, object_name: &str, expires: u64) -> Result<String, OssError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let signed = self.sign_path(Method::GET, object_name, now + expires);
        let sign = self.build_request(Method::GET, object_name, HeaderMap::new(), &signed)?;
        Ok(sign.0)
    }

    pub async fn put_object(
        &self,
        buf: &[u8],
        object_name: &str,
        headers: HeaderMap,
        resources: &HashMap<String, String>,
    ) -> Result<(), OssError> {
        let (host, headers) = self.build_request(Method::PUT, object_name, headers, resources)?;

        let resp = self.http_client.put(&host).headers(headers).body(buf.to_owned()).send().await?;

        if resp.status().is_success() {
            Ok(())
        }
        else {
            Err(OssError::Object(ObjectError::DeleteError {
                msg: format!("can not put object, status code, status code: {}", resp.status()).into(),
            }))
        }
    }

    pub fn put_object_url(&self, object_name: &str, expires: u64) -> Result<String, OssError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        println!("now: {}", now);
        let signed = self.sign_path(Method::PUT, object_name, now + expires);
        let sign = self.build_request(Method::PUT, object_name, HeaderMap::new(), &signed)?;
        Ok(sign.0)
    }

    pub async fn copy_object_from_object(
        &self,
        src: &str,
        dest: &str,
        headers: HeaderMap,
        resources: &HashMap<String, String>,
    ) -> Result<(), OssError> {
        let (host, mut headers) = self.build_request(Method::PUT, dest, headers, resources)?;
        headers.insert("x-oss-copy-source", src.parse()?);

        let resp = self.http_client.put(&host).headers(headers).send().await?;
        if resp.status().is_success() {
            Ok(())
        }
        else {
            Err(OssError::Object(ObjectError::CopyError {
                msg: format!("can not copy object, status code: {}", resp.status()).into(),
            }))
        }
    }

    pub async fn delete_object<S>(&self, object_name: &str) -> Result<(), OssError> {
        let headers = HeaderMap::new();
        let (host, headers) = self.build_request(Method::DELETE, object_name, headers, &HashMap::new())?;

        let resp = self.http_client.delete(&host).headers(headers).send().await?;

        if resp.status().is_success() {
            Ok(())
        }
        else {
            Err(OssError::Object(ObjectError::DeleteError {
                msg: format!("can not delete object, status code: {}", resp.status()).into(),
            }))
        }
    }

    pub async fn head_object<S>(&self, object_name: &str) -> Result<ObjectMeta, OssError> {
        let (host, headers) = self.build_request(Method::HEAD, object_name, HeaderMap::new(), &HashMap::new())?;

        let resp = self.http_client.head(&host).headers(headers).send().await?;

        if resp.status().is_success() {
            Ok(ObjectMeta::from_header_map(resp.headers())?)
        }
        else {
            Err(OssError::Object(ObjectError::DeleteError {
                msg: format!("can not head object, status code: {}", resp.status()).into(),
            }))
        }
    }
    /// Notify oss to init a Multipart Upload event
    pub async fn init_multi(
        &self,
        object_name: &str,
        headers: HeaderMap,
        resources: &HashMap<String, String>,
    ) -> Result<InitiateMultipartUploadResult, OssError> {
        let (host, headers) = self.build_request(Method::POST, object_name, headers, resources)?;

        let resp = self.http_client.post(&host).headers(headers).send().await?;

        if resp.status().is_success() {
            let body = resp.text().await?;
            let res = quick_xml::de::from_str::<InitiateMultipartUploadResult>(&body)?;
            Ok(res)
        }
        else {
            Err(OssError::Object(ObjectError::PostError {
                msg: format!("init multi failed, status code, status code: {}", resp.status()).into(),
            }))
        }
    }
    /// Upload data in chunks according to the specified Object name and uploadId
    pub async fn upload_part(
        &self,
        buf: &[u8],
        object_name: &str,
        headers: HeaderMap,
        resources: &HashMap<String, String>,
    ) -> Result<String, OssError> {
        let (host, headers) = self.build_request(Method::PUT, object_name, headers, resources)?;

        let resp = self.http_client.put(&host).headers(headers).body(buf.to_owned()).send().await?;

        if resp.status().is_success() {
            let e_tag = resp.headers().get("ETag").unwrap().to_str().unwrap();
            Ok(e_tag.to_string())
        }
        else {
            Err(OssError::Object(ObjectError::PutError {
                msg: format!("can not put object, status code, status code: {}", resp.status()).into(),
            }))
        }
    }
    /// Complete the multipart upload of the entire file
    ///
    /// body format
    /// <CompleteMultipartUpload>
    /// <Part>
    /// <PartNumber>PartNumber</PartNumber>
    /// <ETag>ETag</ETag>
    /// </Part>
    /// ...
    /// </CompleteMultipartUpload>
    ///
    /// # Examples
    ///
    ///  #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    ///  #[serde(rename_all = "PascalCase")]
    ///  pub struct PartWrapper {
    ///      pub part: Vec<Part>,
    ///  }
    ///
    ///  #[derive(Debug, Serialize, Deserialize, PartialEq)]
    ///  #[serde(rename_all = "PascalCase")]
    ///  pub struct Part {
    ///      part_number: usize,
    ///      e_tag: String,
    ///  }
    ///
    ///  let parts = CompleteDTO {
    ///      part: vec![Part {
    ///          part_number: 1,
    ///          e_tag: "50BE5FACC702C5B945588031C6*****".to_string(),
    ///      }],
    ///  };
    ///
    ///  let body = quick_xml::se::to_string_with_root("CompleteMultipartUpload", &parts).unwrap();
    pub async fn complete_multi(
        &self,
        body: String,
        object_name: &str,
        headers: HeaderMap,
        resources: &HashMap<String, String>,
    ) -> Result<CompleteMultipartUploadResult, OssError> {
        let (host, headers) = self.build_request(Method::POST, object_name, headers, resources)?;

        let resp = self.http_client.post(&host).headers(headers).body(body).send().await?;

        if resp.status().is_success() {
            let body = resp.text().await?;
            let res = quick_xml::de::from_str::<CompleteMultipartUploadResult>(&body)?;
            Ok(res)
        }
        else {
            Err(OssError::Object(ObjectError::PostError {
                msg: format!("complete multi failed, status code, status code: {}", resp.status()).into(),
            }))
        }
    }
    /// Cancel the MultipartUpload event and delete the corresponding Part data
    pub async fn abort_multi(
        &self,
        object_name: &str,
        headers: HeaderMap,
        resources: &HashMap<String, String>,
    ) -> Result<(), OssError> {
        let (host, headers) = self.build_request(Method::DELETE, object_name, headers, resources)?;

        let resp = self.http_client.delete(&host).headers(headers).send().await?;

        if resp.status().is_success() {
            Ok(())
        }
        else {
            Err(OssError::Object(ObjectError::DeleteError {
                msg: format!("abort multi failed, status code, status code: {}", resp.status()).into(),
            }))
        }
    }
}
