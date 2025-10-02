use crate::models::JwtClaims;
use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    algorithm: Algorithm,
}

impl JwtService {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
            algorithm: Algorithm::HS256,
        }
    }

    pub fn create_access_token(&self, user_id: Uuid, tenant_id: Uuid, roles: Vec<String>, permissions: Vec<String>) -> Result<String> {
        let now = Utc::now();
        let claims = JwtClaims {
            sub: user_id,
            tenant_id,
            iat: now.timestamp(),
            exp: (now + Duration::hours(8)).timestamp(),
            aud: "aion-r".to_string(),
            iss: "aion-r-auth".to_string(),
            roles,
            permissions,
            session_id: Uuid::new_v4(),
        };

        let header = Header::new(self.algorithm);
        let token = encode(&header, &claims, &self.encoding_key)?;
        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<JwtClaims> {
        let validation = Validation::new(self.algorithm);
        let token_data = decode::<JwtClaims>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}