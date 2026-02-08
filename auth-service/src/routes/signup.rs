use crate::domain::User;
use crate::services::hashmap_user_store::UserStoreError;
use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let mut read_store = state.user_store.write().await;

    let user = User::new(request.email, request.password, request.requires_2fa);
    match read_store.add_user(user) {
        Ok(_) => {
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });
            (StatusCode::CREATED, response)
        }
        Err(error) => {
            let response = Json(SignupResponse {
                message: format!(
                    "{}",
                    match error {
                        UserStoreError::UserAlreadyExists => {
                            "User already exists"
                        }
                        UserStoreError::UserNotFound => {
                            "User not found"
                        }
                        UserStoreError::InvalidCredentials => {
                            "Invalid credentials"
                        }
                    }
                ),
            });
            (StatusCode::UNPROCESSABLE_ENTITY, response)
        }
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
