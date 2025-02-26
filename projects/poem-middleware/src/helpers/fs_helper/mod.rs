use crate::{YxError, identifiers::AttachmentId};
use chrono::{Datelike, Timelike};
use poem_openapi::{Object, types::multipart::Upload};
use std::{
    hash::{DefaultHasher, Hasher},
    path::Path,
};

#[derive(Object)]
pub struct UploadCallback {
    /// 文件编号
    pub file_id: AttachmentId,
    /// 文件大小
    pub file_size: usize,
    ///
    pub file_path: String,
}
pub async fn save_upload(file: Upload, root: &Path) -> Result<UploadCallback, YxError> {
    let file_id = AttachmentId::now();
    let file_name = hash_file_name(file.file_name().unwrap_or_default());
    let size = file.size();
    let mut input = file.into_async_read();
    let file_path = root.join(&file_name);
    let mut output = if file_path.exists() {
        return Err(YxError::service_error(format!("File `{}` already exists!", file_path.display())));
    }
    else {
        tokio::fs::File::create(&file_path).await?
    };
    tokio::io::copy(&mut input, &mut output).await?;
    let file_path = match file_path.canonicalize() {
        Ok(o) => o.to_string_lossy().to_string(),
        Err(_) => String::new(),
    };
    Ok(UploadCallback { file_id, file_size: size, file_path })
}

/// name-with-date, yyyy-MM-DD-HH-mm-ss-namehash.extension
fn hash_file_name(file_name: &str) -> String {
    let ext = Path::new(file_name).extension().and_then(|e| e.to_str()).unwrap_or_default();
    let mut hasher = DefaultHasher::new();
    file_name.hash(&mut hasher);
    let name_hash = hasher.finish();
    let now = chrono::Local::now();
    // name-with-date, yyyy-MM-DD-HH-mm-ss-name_hash.extension
    format!(
        "{}-{:02}-{:02}-{:02}-{:02}-{:02}-{:x}.{}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        name_hash,
        ext
    )
}
