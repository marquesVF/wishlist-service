use revolt_rocket_okapi::JsonSchema;

#[derive(serde::Serialize, JsonSchema)]
pub struct Wishlist {
    pub name: String,
}
