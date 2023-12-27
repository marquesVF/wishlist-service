pub mod get_wishlist;
pub mod post_wishlist;
pub mod put_wishlist;

use axum::{
    routing::{get, post},
    Router,
};

use self::{
    get_wishlist::{get_wishlist_by_id, get_wishlists_from_user},
    post_wishlist::post_wishlist,
    put_wishlist::put_item_in_wishlist,
};

#[derive(Clone)]
struct AppState { }

pub fn register_routes() -> Router {
    let state = AppState {};

    Router::new()
        .route(
            "/wishlists/from_user/:user_id",
            get(get_wishlists_from_user),
        )
        .route("/wishlists", post(post_wishlist))
        .route(
            "/wishlists/:wishlist_id",
            get(get_wishlist_by_id).put(put_item_in_wishlist),
        )
        .with_state(state)
}
