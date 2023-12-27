use crate::tables::WishlistTableEntry;
use sqlx::{Pool, Postgres};
use wishlist::Product;

pub async fn insert_wishlist(
    name: &str,
    user_id: &str,
    pool: &Pool<Postgres>,
) -> Result<i32, sqlx::Error> {
    let record = sqlx::query!(
        r#"INSERT INTO Wishlist (name, user_id) VALUES ($1, $2) RETURNING id"#,
        name,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}

pub async fn select_wishlist_by_user_id(
    user_id: &str,
    pool: &Pool<Postgres>,
) -> Result<Vec<WishlistTableEntry>, sqlx::Error> {
    sqlx::query_as!(
        WishlistTableEntry,
        r#"SELECT id, name, user_id FROM Wishlist WHERE user_id = $1"#,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn select_wishlist_by_id(
    wishlist_id: &i32,
    pool: &Pool<Postgres>,
) -> Result<WishlistTableEntry, sqlx::Error> {
    sqlx::query_as!(
        WishlistTableEntry,
        r#"SELECT id, name, user_id FROM Wishlist WHERE id = $1"#,
        wishlist_id
    )
    .fetch_one(pool)
    .await
}

pub async fn select_wishlist_products(
    wishlist_id: &i32,
    pool: &Pool<Postgres>,
) -> Result<Vec<Product>, sqlx::Error> {
    sqlx::query_as!(
        Product,
        r#"
          SELECT
            Product.name, Product.sku, Product.price_in_cents
          FROM
            Product
          JOIN
            WishlistHasProducts
          ON
            WishlistHasProducts.product_sku = Product.sku
          WHERE
            WishlistHasProducts.wishlist_id = $1
        "#,
        wishlist_id
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_product_into_wishlist(
    wishlist_id: &i32,
    product_sku: &str,
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO WishlistHasProducts (wishlist_id, product_sku) VALUES ($1, $2)"#,
        wishlist_id,
        product_sku
    )
    .execute(pool)
    .await?;

    Ok(())
}
