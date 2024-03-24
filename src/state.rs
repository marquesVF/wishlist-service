use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

///
#[derive(Clone)]
pub struct ServerState {
    pub db_pool: Pool<Postgres>,
}

impl ServerState {
    pub async fn new(database_url: &str) -> ServerState {
        let db_pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
            .unwrap();

        ServerState { db_pool }
    }
}

pub async fn configure_app_state() -> ServerState {
    let db_url =
        std::env::var("DATABASE_URL").expect("'DATABASE_URL' is a required environment variable");

    ServerState::new(&db_url).await
}
