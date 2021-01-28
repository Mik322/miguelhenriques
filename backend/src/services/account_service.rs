use crate::{
    errors::ServiceError,
    models::{
        user::{User, UserLoginData},
        user_token::UserToken,
    },
    Pool,
};
use actix_web::{http::StatusCode, web};
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
