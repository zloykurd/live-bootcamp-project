use crate::domain::data_stores::UserStoreError;
use crate::domain::error::AuthApiError;
use crate::domain::User;
use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use email_address::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthApiError> {
    if EmailAddress::from_str(request.email.as_str()).is_err() {
        return Err(AuthApiError::InvalidCredentials);
    }

    if request.password.len() < 8 {
        return Err(AuthApiError::InvalidCredentials);
    }

    let mut store = state.user_store.write().await;
    let user = User::new(request.email, request.password, request.requires_2fa);
    match store.add_user(user).await {
        Ok(_) => {
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });
            Ok((StatusCode::CREATED, response))
        }
        Err(error) => match error {
            UserStoreError::UserAlreadyExists => Err(AuthApiError::UserAlreadyExists),
            UserStoreError::InvalidCredentials => Err(AuthApiError::InvalidCredentials),
            _ => Err(AuthApiError::InternalError),
        },
    }
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}
