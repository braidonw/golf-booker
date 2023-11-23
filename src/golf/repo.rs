use super::GolfEvent;

struct Repo {
    pool: sqlx::SqlitePool,
}

pub async fn store_event(event: GolfEvent) -> anyhow::Result<()> {
    todo!()
}
