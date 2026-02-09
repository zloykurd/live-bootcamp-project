use std::error::Error;

use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use axum::{routing::post, serve::Serve, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;

use crate::domain::error::AuthApiError;
pub use app_state::{AppState, UserStore};

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let asset_dir =
            ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

        let router = Router::new()
            .fallback_service(asset_dir)
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify_2fa", post(routes::verify_2fa))
            .route("/varify_token", post(routes::varify_token))
            .route("/signup", post(routes::signup))
            .with_state(app_state);

        let listener = TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl IntoResponse for AuthApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthApiError::UserAlreadyExists => {
                (StatusCode::CONFLICT, String::from("User already exists"))
            }
            AuthApiError::InvalidCredentials => {
                (StatusCode::BAD_REQUEST, String::from("Invalid credentials"))
            }
            AuthApiError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, String::from("Unexpected error"))
            }
        };
        let body = Json(ErrorResponse {
            message: error_message.to_string(),
        });
        (status, body).into_response()
    }
}
