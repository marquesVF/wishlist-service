use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Wishlist {
    pub id: u32,
    pub name: String,
    pub user_id: String,
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Product {
    pub sku: String,
    pub price_in_cents: u32,
}
