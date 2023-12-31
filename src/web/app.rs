use super::{event, event_group};
use crate::{
    golf,
    users::{self, Backend},
    web::{auth, protected},
};
use axum::Router;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError};
use axum_login::{
    login_required,
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::net::SocketAddr;
use std::sync::Arc;
use time::Duration;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

pub struct App {
    db: SqlitePool,
    golf_client: golf::GolfClient,
}

pub struct AppState {
    pub golf_client: golf::GolfClient,
    pub db: SqlitePool,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Init and seed db
        let db_filename = "golf.db";
        let db_options = SqliteConnectOptions::new()
            .filename(db_filename)
            .create_if_missing(true);
        let db = SqlitePool::connect_with(db_options).await?;
        sqlx::migrate!().run(&db).await?;

        // If no users exist, seed from environment
        if !users::do_users_exist(&db).await? {
            users::seed_from_environment(&db).await?;
        }

        // Golf client to interact with the club website
        let base_path = std::env::var("GOLF_BASE_PATH").expect("GOLF_BASE_PATH not set");
        let golf_client = golf::GolfClient::new(&base_path, &db);

        Ok(Self { db, golf_client })
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()?;

        dbg!(&port);
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

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

        // TODO: Should try and figure out how to use the same backend as the session
        let backend = Backend::new(self.db.clone());
        let auth_service = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|_: BoxError| async {
                StatusCode::BAD_REQUEST
            }))
            .layer(AuthManagerLayerBuilder::new(backend, session_layer).build());

        // Static Files
        let static_files_service = ServeDir::new("assets");

        // GolfClient
        let golf_client = self.golf_client;

        let state = Arc::new(AppState {
            golf_client,
            db: self.db,
        });

        let app = Router::new()
            .merge(protected::router(state.clone()))
            .merge(event::router(state.clone()))
            .merge(event_group::router(state.clone()))
            .route_layer(login_required!(Backend, login_url = "/login"))
            .merge(auth::router())
            .layer(auth_service)
            .nest_service("/assets", static_files_service);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
