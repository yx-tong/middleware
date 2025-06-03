use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use poem::{Request, http::StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use std::{ops::Add, time::Duration};

pub fn jwt_encode<T: Serialize>(claims: T, private_key: &[u8]) -> poem::Result<String> {
    let header = jsonwebtoken::Header::new(Algorithm::ES256);
    let key = EncodingKey::from_ec_pem(private_key).expect("非法加密密钥");
    match jsonwebtoken::encode(&header, &claims, &key) {
        Ok(o) => Ok(o),
        Err(e) => Err(poem::Error::from_string(e.to_string(), StatusCode::NOT_ACCEPTABLE)),
    }
}

pub fn jwt_decode<T: DeserializeOwned>(token: &str, public_key: &[u8]) -> poem::Result<T> {
    let mut method = Validation::new(Algorithm::ES256);
    method.validate_exp = true;
    method.validate_aud = false;
    let key = DecodingKey::from_ec_pem(public_key).expect("非法解密密钥");
    match jsonwebtoken::decode::<T>(&token, &key, &method) {
        Ok(o) => Ok(o.claims),
        Err(e) => Err(poem::Error::from_string(format!("Illegal request: {}", e), StatusCode::UNAUTHORIZED)),
    }
}
pub fn jwt_request<T: DeserializeOwned>(input: &Request, public_key: &[u8]) -> poem::Result<T> {
    match input.header("Authorization") {
        Some(s) => jwt_decode(s.trim_start_matches("Bearer "), public_key),
        None => Err(poem::Error::from_string("Illegal request: Missing `Authorization`", StatusCode::UNAUTHORIZED)),
    }
}

pub fn jwt_time(time: f32) -> poem::Result<u64> {
    match std::time::SystemTime::now().add(Duration::from_secs_f32(time)).duration_since(std::time::UNIX_EPOCH) {
        Ok(o) => Ok(o.as_secs()),
        Err(_) => Err(poem::Error::from_string("Illegal time service center.".to_string(), StatusCode::CONFLICT)),
    }
}
