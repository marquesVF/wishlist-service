mod db_provider;
mod routes;
mod state;
mod swagger;
mod wishlist;

use dotenv::dotenv;
use routes::register_routes;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use swagger::register_swagger;
use tower::ServiceBuilder;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::configure_app_state;

#[tokio::main]
async fn main() {
    dotenv().expect("unable to load .env file");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            // If no log level is informed, debug is used as default
            std::env::var("LOG_LEVEL").unwrap_or_else(|_| "tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let server_state = configure_app_state().await;
    let router = register_routes(server_state);
    let router = register_swagger(router)
        // Using tower to add tracing layer
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on http://{}", addr);
    println!("  📃️ http://{}/docs", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
