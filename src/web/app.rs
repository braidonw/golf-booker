use crate::{
    golf,
    users::Backend,
    web::{auth, protected},
};
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Extension};
use axum_login::{
    login_required,
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::sync::Arc;
use time::Duration;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

use super::event;

pub struct App {
    db: SqlitePool,
    golf_client: golf::GolfClient,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = SqlitePool::connect(":memory:").await?;
        sqlx::migrate!().run(&db).await?;

        // Golf client to interact with the club website
        let base_path = std::env::var("GOLF_BASE_PATH").expect("GOLF_BASE_PATH not set");
        let golf_client = golf::GolfClient::new(&base_path, &db);

        Ok(Self { db, golf_client })
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        // Session layer.
        //
        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)));

        // Auth service.
        //
        // This combines the session layer with our backend to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = Backend::new(self.db);
        let auth_service = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|_: BoxError| async {
                StatusCode::BAD_REQUEST
            }))
            .layer(AuthManagerLayerBuilder::new(backend, session_layer).build());

        // Static Files
        let static_files_service = ServeDir::new("assets");

        // GolfClient
        let golf_client = Arc::new(self.golf_client);

        let app = protected::router()
            .merge(event::router())
            .route_layer(login_required!(Backend, login_url = "/login"))
            .merge(auth::router())
            .layer(auth_service)
            .nest_service("/assets", static_files_service)
            .layer(Extension(golf_client));

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
