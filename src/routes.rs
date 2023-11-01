pub mod get_wishlist;
pub mod post_wishlist;
pub mod put_wishlist;

use axum::{
    routing::{get, put},
    Router,
};

use self::{
    get_wishlist::get_wishlists_from_user, post_wishlist::post_wishlist,
    put_wishlist::put_item_in_wishlists,
};

pub fn register_routes(router: Router) -> Router {
    router
        .route(
            "/wishlists/from_user/:user_id",
            get(get_wishlists_from_user).post(post_wishlist),
        )
        .route("/wishlists/:wishlist_id", put(put_item_in_wishlists))
}
