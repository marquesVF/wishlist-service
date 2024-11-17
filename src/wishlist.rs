use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Wishlist {
    pub id: i32,
    pub name: String,
    pub user_id: String,
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub sku: String,
    pub name: String,
    pub price_in_cents: i32,
}
