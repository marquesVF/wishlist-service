mod routes;
mod state;
mod swagger;

use routes::register_routes;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use swagger::register_swagger;
use tower::ServiceBuilder;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::configure_app_state;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            // FIXME remove hard coded default at `unwrap_or_else`
            std::env::var("RUST_LOG").unwrap_or_else(|_| "todo_axum=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = configure_app_state().await;
    let router = register_routes(app_state);
    let app = register_swagger(router)
        // Using tower to add tracing layer
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on http://{}", addr);
    println!("  📃️ http://{}/docs", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
