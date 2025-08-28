use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Duration, Utc};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub username: String,
    pub exp: usize, // timestamp de expiración
    pub iat: usize, // timestamp de emisión
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new() -> Self {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "pitaia-super-secret-key-change-in-production".to_string());
        
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }
    
    pub fn generate_token(&self, user_id: Uuid, username: &str) -> Result<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            exp: (now + Duration::hours(24)).timestamp() as usize,
            iat: now.timestamp() as usize,
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| anyhow::anyhow!("Error generando token: {}", e))
    }
    
    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| anyhow::anyhow!("Token inválido: {}", e))
    }
}

pub fn hash_password(password: &str) -> Result<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| anyhow::anyhow!("Error hasheando password: {}", e))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    bcrypt::verify(password, hash)
        .map_err(|e| anyhow::anyhow!("Error verificando password: {}", e))
}
