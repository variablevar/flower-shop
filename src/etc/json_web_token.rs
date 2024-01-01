use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::strings::SECRATE_KEY;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct JwtService;

impl JwtService {
    pub fn generate_token(subject: &str, expiration_secs: u64) -> String {
        let claims = Claims {
            sub: subject.to_owned(),
            exp: (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
                + expiration_secs) as usize,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRATE_KEY.as_bytes()),
        )
        .unwrap()
    }

    pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(SECRATE_KEY.as_bytes()),
            &Validation::default(),
        );
        match token_data {
            Ok(t) => Ok(t.claims),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_sign_and_verify_json_web_token() {
        let payload = "Cool";

        // Generate a token
        let token = JwtService::generate_token(payload, 3600);

        // Verify and decode the token
        match JwtService::verify_token(&token) {
            Ok(claims) => assert_eq!(claims.sub, payload),
            Err(e) => eprintln!("Failed to decode token: {:?}", e),
        }
    }
}
