use crate::domain::data_stores::UserStoreError;
use crate::domain::error::AuthApiError;
use crate::domain::User;
use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthApiError> {
    let user_result = User::new(request.email, request.password, request.requires_2fa);
    let user= match user_result {
        Ok(user) => user,
        Err(_) => {
            return Err(AuthApiError::InvalidCredentials);
        }
    };
    let mut store = state.user_store.write().await;
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
