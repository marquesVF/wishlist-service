use rusqlite::params;
use tokio_rusqlite::Connection;

use crate::queries::{SELECT_WISHLIST, SELECT_WISHLIST_PRODUCTS};
use wishlist::{Product, Wishlist};

pub async fn get_wishlist(wishlist_id: String) -> Wishlist {
    let conn = Connection::open("data.db").await.unwrap();

    conn.call(move |conn| {
        let mut wishlist = conn.query_row(SELECT_WISHLIST, params![wishlist_id], |r| {
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
