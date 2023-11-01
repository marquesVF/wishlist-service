// TODO remove duplication in this file
mod queries;

use async_trait::async_trait;
use queries::{CREATE_WISHLIST, SELECT_WISHLIST, SELECT_WISHLIST_PRODUCTS};
use rusqlite::params;
use tokio_rusqlite::Connection;
use wishlist::{Product, Wishlist};

use crate::queries::{INSERT_PRODUCT_INTO_WISHLIST, SELECT_WISHLIST_BY_USER_ID};

pub struct SQLiteProvider {
    conn: Connection,
}

impl SQLiteProvider {
    pub async fn new(address: &str) -> SQLiteProvider {
        SQLiteProvider {
            conn: Connection::open(address).await.unwrap(),
        }
    }
}

#[async_trait]
pub trait WishlistProvider {
    async fn create_wishlist(&self, name: String, user_id: String) -> Wishlist;
    async fn get_user_wishlists(&self, user_id: String) -> Vec<Wishlist>;
    async fn add_product_to_wishlist(&self, wishlist_id: String, product_sku: String) -> Wishlist;
}

#[async_trait]
impl WishlistProvider for SQLiteProvider {
    async fn create_wishlist(&self, name: String, user_id: String) -> Wishlist {
        create_wishlist(name, user_id).await
    }

    async fn get_user_wishlists(&self, user_id: String) -> Vec<Wishlist> {
        get_user_wishlists(user_id).await
    }

    async fn add_product_to_wishlist(&self, wishlist_id: String, product_sku: String) -> Wishlist {
        add_product_to_wishlist(wishlist_id, product_sku).await
    }
}

async fn foo() {
    let provider = SQLiteProvider::new("data.db").await;
    let x = provider
        .create_wishlist("foo".to_string(), "b".to_string())
        .await;
}

pub async fn create_wishlist(name: String, user_id: String) -> Wishlist {
    let conn: Connection = Connection::open("data.db").await.unwrap();

    let wishlist = conn
        .call(move |conn| {
            conn.execute(CREATE_WISHLIST, params![name, user_id])?;

            conn.query_row(SELECT_WISHLIST_BY_USER_ID, params![user_id], |r| {
                Ok(Wishlist {
                    id: r.get(0).unwrap(),
                    user_id: r.get(2).unwrap(),
                    name: r.get(1).unwrap(),
                    products: vec![],
                })
            })
        })
        .await
        .unwrap();

    wishlist
}

pub async fn get_user_wishlists(user_id: String) -> Vec<Wishlist> {
    let conn: Connection = Connection::open("data.db").await.unwrap();

    let res = conn
        .call(move |conn| {
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
                            name: r.get(0).unwrap(),
                            sku: r.get(1).unwrap(),
                            price_in_cents: r.get(2).unwrap(),
                        })
                    })?
                    .collect::<Result<Vec<Product>, rusqlite::Error>>()?;

                wishlist.products = products;
            }

            Ok(wishlists)
        })
        .await
        .unwrap();

    res
}

pub async fn add_product_to_wishlist(wishlist_id: String, product_sku: String) -> Wishlist {
    let conn = Connection::open("data.db").await.unwrap();

    conn.call(move |conn| {
        conn.execute(
            INSERT_PRODUCT_INTO_WISHLIST,
            params![wishlist_id, product_sku],
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
