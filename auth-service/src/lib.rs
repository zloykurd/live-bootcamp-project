use std::error::Error;

use axum::{routing::post, serve::Serve, Router};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

pub mod routes;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let asset_dir =
            ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

        let router = Router::new()
            .fallback_service(asset_dir)
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify_2fa", post(routes::verify_2fa))
            .route("/varify_token", post(routes::varify_token))
            .route("/signup", post(routes::signup));

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
