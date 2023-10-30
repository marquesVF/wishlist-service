mod swagger;

use revolt_rocket_okapi::{openapi, openapi_get_routes, JsonSchema};
use rocket::serde::json::Json;
use rocket::{get, launch};
use swagger::register_swagger;

#[derive(serde::Serialize, JsonSchema)]
struct Response {
    reply: String,
}

#[openapi(tag = "Wishlist")]
#[get("/")]
fn wishlists() -> Json<Response> {
    Json(Response {
        reply: "show me the docs!".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build()
        .mount("/wishlists", openapi_get_routes![wishlists]);

    register_swagger(rocket)
}
