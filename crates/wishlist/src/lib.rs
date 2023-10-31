use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Wishlist {
    pub name: String,
    pub user_id: String,
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Product {
    pub sku: String,
    pub name: String,
    pub price_in_cents: u32,
}
