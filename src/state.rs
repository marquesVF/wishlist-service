use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

impl AppState {
    pub async fn new(database_url: &str) -> AppState {
        let db_pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
            .unwrap();

        AppState { db_pool }
    }
}

pub async fn configure_app_state() -> AppState {
    dotenv().unwrap();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    AppState::new(&db_url).await
}
