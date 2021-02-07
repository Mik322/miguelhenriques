use crate::{
    errors::ServiceError,
    models::{
        user::{User, UserLoginData},
        user_token::UserToken,
    },
    Pool,
};
use actix_web::{
    http::{HeaderValue, StatusCode},
    web,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenBody {
    pub token: String,
    pub token_type: String,
}

pub fn login(
    pool: web::Data<Pool>,
    login_data: web::Json<UserLoginData>,
) -> Result<TokenBody, ServiceError> {
    match User::login_user(pool, login_data.into_inner()) {
        Some(session) => {
            match serde_json::from_value(
                json!({"token": UserToken::generate(&session), "token_type": "bearer"}),
            ) {
                Ok(token) => {
                    if session.login_session == "" {
                        Err(ServiceError::new(
                            StatusCode::UNAUTHORIZED,
                            "Login has failed".to_string(),
                        ))
                    } else {
                        Ok(token)
                    }
                }
                Err(_) => Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "There was a problem while loggin in".to_string(),
                )),
            }
        }
        None => Err(ServiceError::new(
            StatusCode::UNAUTHORIZED,
            "User not found".to_string(),
        )),
    }
}

pub fn logout(auth_header: &HeaderValue, pool: web::Data<Pool>) -> Result<(), ()> {
    if let Ok(header_str) = auth_header.to_str() {
        if !header_str.starts_with("bearer") && !header_str.starts_with("Bearer") {
            return Err(());
        };
        let token_str = header_str[6..auth_header.len()].trim();
        match UserToken::decode_token(token_str.to_string()) {
            Ok(token_data) => {
                let conn = pool.get().unwrap();
                let token = token_data.claims;
                if User::logout_user(&conn, token.username) {
                    Ok(())
                } else {
                    Err(())
                }
            }
            Err(_) => Err(()),
        }
    } else {
        Err(())
    }
}
