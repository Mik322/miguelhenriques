use super::user::UserSession;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub static KEY: [u8; 32] = *include_bytes!("../secret.key");
static TIME_TO_EXPIRE: u64 = 7 * 24 * 60 * 60; //One week in seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    //issued at
    iat: u64,
    //expiration time
    exp: u64,

    user: String,
    session: String,
}

impl UserToken {
    pub fn generate(user_session: &UserSession) -> String {
        let now = Utc::now().timestamp() as u64;
        let user = Self {
            iat: now,
            exp: now + TIME_TO_EXPIRE,
            user: user_session.username.clone(),
            session: user_session.login_session.clone(),
        };

        encode(&Header::default(), &user, &EncodingKey::from_secret(&KEY)).unwrap()
    }
}
