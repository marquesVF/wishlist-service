mod routes;
mod swagger;
mod wishlists;

use routes::register_routes;
use std::net::SocketAddr;

use axum::Router;
use swagger::register_swagger;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let router = register_routes(Router::new());
    let app = register_swagger(router);

    // TODO investigate better logging
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
