use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use wishlist::Wishlist;

use crate::queries::select_wishlist_products;

#[derive(Serialize, Deserialize, Debug)]
pub struct WishlistTableEntry {
    pub id: i32,
    pub name: String,
    pub user_id: String,
}

impl WishlistTableEntry {
    pub async fn to_wishlist(self, pool: &Pool<Postgres>) -> Result<Wishlist, sqlx::Error> {
        let products = select_wishlist_products(&self.id, &pool).await?;

        Ok(Wishlist {
            id: self.id,
            name: self.name,
            user_id: self.user_id,
            products,
        })
    }
}
