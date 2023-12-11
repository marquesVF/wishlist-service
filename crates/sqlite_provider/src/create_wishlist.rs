use rusqlite::params;
use tokio_rusqlite::Connection;

use crate::queries::{CREATE_WISHLIST, SELECT_WISHLIST_PRODUCTS};
use crate::queries::{INSERT_PRODUCT_INTO_WISHLIST, SELECT_WISHLIST_BY_USER_ID};
use wishlist::{Product, Wishlist};

pub async fn create_wishlist(name: String, product_sku: String, user_id: String) -> Wishlist {
    let conn = Connection::open("data.db").await.unwrap();

    conn.call(move |conn| {
        conn.execute(CREATE_WISHLIST, params![name, user_id])?;
        let new_wishlist_id = conn.last_insert_rowid();
        conn.execute(
            INSERT_PRODUCT_INTO_WISHLIST,
            params![new_wishlist_id, product_sku],
        )?;

        let mut wishlist = conn.query_row(SELECT_WISHLIST_BY_USER_ID, params![user_id], |r| {
            Ok(Wishlist {
                id: r.get(0).unwrap(),
                user_id: r.get(2).unwrap(),
                name: r.get(1).unwrap(),
                products: vec![],
            })
        })?;

        let mut stmt = conn.prepare(SELECT_WISHLIST_PRODUCTS).unwrap();
        let products = stmt
            .query_map(params![&wishlist.id], |r| {
                Ok(Product {
                    name: r.get(0).unwrap(),
                    sku: r.get(1).unwrap(),
                    price_in_cents: r.get(2).unwrap(),
                })
            })?
            .collect::<Result<Vec<Product>, rusqlite::Error>>()?;

        wishlist.products = products;

        Ok(wishlist)
    })
    .await
    .unwrap()
}
