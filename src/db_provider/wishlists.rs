use sqlx::{Pool, Postgres};

use crate::wishlist::Wishlist;

use super::queries::{
    insert_product_into_wishlist, insert_wishlist, select_wishlist_by_id,
    select_wishlist_by_user_id, select_wishlist_products,
};

pub async fn get_user_wishlists(
    user_id: &str,
    pool: &Pool<Postgres>,
) -> Result<Vec<Wishlist>, String> {
    let wishlist_table_entries = select_wishlist_by_user_id(user_id, &pool)
        .await
        .map_err(|e| e.to_string())?;
    let mut wishlists = vec![];
    for wishlist in wishlist_table_entries {
        let wishlist = wishlist
            .to_wishlist(&pool)
            .await
            .map_err(|e| e.to_string())?;

        wishlists.push(wishlist);
    }

    Ok(wishlists)
}

pub async fn get_wishlist(
    wishlist_id: &i32,
    pool: &Pool<Postgres>,
) -> Result<Wishlist, sqlx::Error> {
    let wishlist = select_wishlist_by_id(wishlist_id, &pool).await?;

    Ok(wishlist.to_wishlist(&pool).await?)
}

pub async fn create_wishlist(
    name: &str,
    user_id: &str,
    product_sku: &str,
    pool: &Pool<Postgres>,
) -> Result<Wishlist, sqlx::Error> {
    let id = insert_wishlist(name, user_id, &pool).await?;
    insert_product_into_wishlist(&id, product_sku, &pool).await?;
    let products = select_wishlist_products(&id, &pool).await?;

    Ok(Wishlist {
        name: name.to_owned(),
        id,
        user_id: user_id.to_owned(),
        products,
    })
}

pub async fn add_product_to_wishlist(
    wishlist_id: &i32,
    product_sku: &str,
    pool: &Pool<Postgres>,
) -> Result<Wishlist, sqlx::Error> {
    insert_product_into_wishlist(wishlist_id, product_sku, &pool).await?;

    let wishlist = select_wishlist_by_id(wishlist_id, &pool).await?;

    Ok(wishlist.to_wishlist(&pool).await?)
}
