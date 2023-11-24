use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    id: i64,
    pub username: String,
    password: String,
}

pub async fn do_users_exist(db: &SqlitePool) -> anyhow::Result<bool> {
    let user_count: i64 = sqlx::query_scalar("select count(*) from users")
        .fetch_one(db)
        .await?;

    Ok(user_count > 0)
}

pub async fn seed_from_environment(db: &SqlitePool) -> anyhow::Result<()> {
    let username = std::env::var("GC_USERNAME").expect("GC_USERNAME not set");
    let password = std::env::var("GC_PASSWORD").expect("GC_PASSWORD not set");
    let hashed_password = password_auth::generate_hash(password);

    sqlx::query!(
        "insert into users (id, username, password) values (?, ?, ?)",
        1,
        username,
        hashed_password,
    )
    .execute(db)
    .await?;

    Ok(())
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
                                 // hash--what this means
                                 // is when the user changes their password the
                                 // auth session becomes invalid.
    }
}

// This allows us to extract the authentication fields from forms. We use this
// to authenticate requests with the backend.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: SqlitePool,
}

impl Backend {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = sqlx::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as("select * from users where username = ? ")
            .bind(creds.username)
            .fetch_optional(&self.db)
            .await?;

        Ok(user.filter(|user| {
            verify_password(creds.password, &user.password)
                .ok()
                .is_some() // We're using password-based authentication--this
                           // works by comparing our form input with an argon2
                           // password hash.
        }))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as("select * from users where id = ?")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<Backend>;
