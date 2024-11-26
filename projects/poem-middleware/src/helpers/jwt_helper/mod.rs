use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use poem::http::StatusCode;
use serde::{Serialize, de::DeserializeOwned};

pub fn jwt_encode<T: Serialize>(claims: T, private_key: &[u8]) -> poem::Result<String> {
    let header = jsonwebtoken::Header::new(Algorithm::ES256);
    let key = EncodingKey::from_ec_pem(private_key).expect("非法加密密钥");
    match jsonwebtoken::encode(&header, &claims, &key) {
        Ok(o) => Ok(o),
        Err(e) => Err(poem::Error::from_string(e.to_string(), StatusCode::NOT_ACCEPTABLE)),
    }
}

pub fn jwt_decode<T: DeserializeOwned>(token: &str, public_key: &[u8]) -> poem::Result<T> {
    let method = Validation::new(Algorithm::ES256);
    let key = DecodingKey::from_ec_pem(public_key).expect("非法解密密钥");
    match jsonwebtoken::decode::<T>(&token, &key, &method) {
        Ok(o) => Ok(o.claims),
        Err(e) => Err(poem::Error::from_string(format!("非法访问: {}", e), StatusCode::UNAUTHORIZED)),
    }
}
