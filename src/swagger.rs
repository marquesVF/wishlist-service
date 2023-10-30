use revolt_rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

use rocket::{Rocket, Build};

pub fn register_swagger(rocket: Rocket<Build>) -> Rocket<Build> {
  rocket.mount("/docs", make_swagger_ui(&get_docs()))
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/wishlists/openapi.json".to_string(),
        ..Default::default()
    }
}
