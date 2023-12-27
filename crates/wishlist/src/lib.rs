use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Wishlist {
    pub id: i32,
    pub name: String,
    pub user_id: String,
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Product {
    pub sku: String,
    pub name: String,
    pub price_in_cents: i32,
}

// pub trait WishlistProvider {
//     fn create_wishlist() -> Wishlist;
//     fn get_user_wishlists() -> Vec<Wishlist>;
//     fn add_product_to_wishlist(String, String) -> Wishlist;
// }
