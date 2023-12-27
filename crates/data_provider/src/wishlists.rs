use sqlx::postgres::PgPoolOptions;

use wishlist::Wishlist;

use crate::queries::{
    insert_product_into_wishlist, insert_wishlist, select_wishlist_by_user_id,
    select_wishlist_products, select_wishlist_by_id,
};

pub async fn get_user_wishlists(user_id: &str) -> Vec<Wishlist> {
    // FIXME make pool a parameter to this function
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:123@localhost/wishlistdb")
        .await
        .unwrap();

    // FIXME handle errors
    let wishlist_table_entries = select_wishlist_by_user_id(user_id, &pool).await.unwrap();
    let mut wishlists = vec![];
    for wishlist in wishlist_table_entries {
        let wishlist = wishlist.to_wishlist(&pool).await.unwrap();

        wishlists.push(wishlist);
    }

    wishlists
}

pub async fn get_wishlist(wishlist_id: &i32) -> Wishlist {
    // FIXME make pool a parameter to this function
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:123@localhost/wishlistdb")
        .await
        .unwrap();

    // FIXME handle errors
    let wishlist = select_wishlist_by_id(wishlist_id, &pool).await.unwrap();
    wishlist.to_wishlist(&pool).await.unwrap()
}

pub async fn create_wishlist(name: &str, user_id: &str, product_sku: &str) -> Wishlist {
    // FIXME make pool a parameter to this function
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:123@localhost/wishlistdb")
        .await
        .unwrap();

    let id = insert_wishlist(name, user_id, &pool).await.unwrap();
    insert_product_into_wishlist(&id, product_sku, &pool)
        .await
        .unwrap();
    let products = select_wishlist_products(&id, &pool).await.unwrap();

    Wishlist {
        name: name.to_owned(),
        id,
        user_id: user_id.to_owned(),
        products,
    }
}

pub async fn add_product_to_wishlist(wishlist_id: &i32, product_sku: &str) -> Wishlist {
    // FIXME make pool a parameter to this function
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:123@localhost/wishlistdb")
        .await
        .unwrap();

    insert_product_into_wishlist(wishlist_id, product_sku, &pool)
        .await
        .unwrap();

    let wishlist = select_wishlist_by_id(wishlist_id, &pool).await.unwrap();

    wishlist.to_wishlist(&pool).await.unwrap()
}

