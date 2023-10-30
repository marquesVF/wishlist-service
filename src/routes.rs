use revolt_rocket_okapi::{openapi, openapi_get_routes};
use rocket::serde::json::Json;
use rocket::{get, Build, Rocket};

use crate::wishlists::Wishlist;

pub fn register_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/wishlists", openapi_get_routes![wishlists])
}

#[openapi(tag = "Wishlist")]
#[get("/")]
fn wishlists() -> Json<Wishlist> {
    Json(Wishlist {
        name: "Some Name".to_string(),
    })
}
