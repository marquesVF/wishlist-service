// TODO remove duplication in this file
mod queries;

use queries::{INSERT_WISHLIST, SELECT_WISHLIST, SELECT_WISHLIST_PRODUCTS};
use rusqlite::params;
use tokio_rusqlite::Connection;
use wishlist::{Product, Wishlist};

use crate::queries::{INSERT_PRODUCT_INTO_WISHLIST, SELECT_WISHLIST_BY_USER_ID};

pub async fn create_wishlist(
    name: String,
    product_sku: String,
    user_id: String,
    price_in_cents: u32,
) -> Wishlist {
    let conn = Connection::open("wishlist.db").await.unwrap();

    conn.call(move |conn| {
        conn.execute(INSERT_WISHLIST, params![name, user_id])?;
        let new_wishlist_id = conn.last_insert_rowid();
        conn.execute(
            INSERT_PRODUCT_INTO_WISHLIST,
            params![new_wishlist_id, product_sku, price_in_cents],
        )?;

        let mut wishlist =
            conn.query_row(SELECT_WISHLIST, params![new_wishlist_id.clone()], |r| {
                Ok(Wishlist {
                    id: r.get(0).unwrap(),
                    user_id: r.get(2).unwrap(),
                    name: r.get(1).unwrap(),
                    products: vec![],
                })
            })?;

        let mut stmt = conn.prepare(SELECT_WISHLIST_PRODUCTS).unwrap();
        let products = stmt
            .query_map(params![new_wishlist_id], |r| {
                Ok(Product {
                    sku: r.get(0).unwrap(),
                    price_in_cents: r.get(0).unwrap(),
                })
            })?
            .collect::<Result<Vec<Product>, rusqlite::Error>>()?;

        wishlist.products = products;

        Ok(wishlist)
    })
    .await
    .unwrap()
}

pub async fn get_wishlist(wishlist_id: String) -> Wishlist {
    let conn = Connection::open("wishlist.db").await.unwrap();

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
                    sku: r.get(0).unwrap(),
                    price_in_cents: r.get(1).unwrap(),
                })
            })?
            .collect::<Result<Vec<Product>, rusqlite::Error>>()?;

        wishlist.products = products;

        Ok(wishlist)
    })
    .await
    .unwrap()
}

pub async fn get_user_wishlists(user_id: String) -> Vec<Wishlist> {
    let conn: Connection = Connection::open("wishlist.db").await.unwrap();

    conn.call(move |conn| {
        let mut stmt = conn.prepare(SELECT_WISHLIST_BY_USER_ID).unwrap();
        let mut wishlists = stmt
            .query_map(params![user_id], |r| {
                Ok(Wishlist {
                    id: r.get(0).unwrap(),
                    user_id: r.get(2).unwrap(),
                    name: r.get(1).unwrap(),
                    products: vec![],
                })
            })?
            .collect::<Result<Vec<Wishlist>, rusqlite::Error>>()?;

        for wishlist in wishlists.iter_mut() {
            let mut stmt = conn.prepare(SELECT_WISHLIST_PRODUCTS).unwrap();
            let products = stmt
                .query_map(params![&wishlist.id], |r| {
                    Ok(Product {
                        sku: r.get(0).unwrap(),
                        price_in_cents: r.get(1).unwrap(),
                    })
                })?
                .collect::<Result<Vec<Product>, rusqlite::Error>>()?;

            wishlist.products = products;
        }

        Ok(wishlists)
    })
    .await
    .unwrap()
}

pub async fn add_product_to_wishlists(
    wishlist_id: String,
    product_sku: String,
    price_in_cents: u32,
) -> Wishlist {
    let conn = Connection::open("wishlist.db").await.unwrap();

    conn.call(move |conn| {
        conn.execute(
            INSERT_PRODUCT_INTO_WISHLIST,
            params![wishlist_id, product_sku, price_in_cents],
        )?;

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
                    sku: r.get(0).unwrap(),
                    price_in_cents: r.get(1).unwrap(),
                })
            })?
            .collect::<Result<Vec<Product>, rusqlite::Error>>()?;

        wishlist.products = products;

        Ok(wishlist)
    })
    .await
    .unwrap()
}
