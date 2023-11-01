use async_trait::async_trait;
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
    pub name: String,
    pub price_in_cents: u32,
}

#[async_trait]
pub trait WishlistProvider {
    async fn create_wishlist(&self, name: String, user_id: String) -> Wishlist;
    async fn get_user_wishlists(&self, user_id: String) -> Vec<Wishlist>;
    async fn add_product_to_wishlist(&self, wishlist_id: String, product_sku: String) -> Wishlist;
}
