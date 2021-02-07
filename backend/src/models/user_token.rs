use super::user::{User, UserSession};
use chrono::Utc;
use diesel::PgConnection;
use jsonwebtoken::{
    decode, encode, errors::Result, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

static KEY: [u8; 32] = *include_bytes!("../secret.key");
static TIME_TO_EXPIRE: u64 = 7 * 24 * 60 * 60; //One week in seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    //issued at
    iat: u64,
    //expiration time
    exp: u64,

    pub username: String,
    pub session: String,
}

impl UserToken {
    pub fn generate(user_session: &UserSession) -> String {
        let now = Utc::now().timestamp() as u64;
        let user = Self {
            iat: now,
            exp: now + TIME_TO_EXPIRE,
            username: user_session.username.clone(),
            session: user_session.login_session.clone(),
        };

        encode(&Header::default(), &user, &EncodingKey::from_secret(&KEY)).unwrap()
    }

    pub fn decode_token(encoded_token: String) -> Result<TokenData<UserToken>> {
        let token = decode::<UserToken>(
            &encoded_token,
            &DecodingKey::from_secret(&KEY),
            &Validation::default(),
        );
        token
    }

    pub fn verify_token(
        token: &TokenData<UserToken>,
        conn: &PgConnection,
    ) -> std::result::Result<String, String> {
        if User::is_valid_session(&token.claims, conn) {
            Ok(token.claims.username.clone())
        } else {
            Err("Token not valid".to_string())
        }
    }
}
