pub mod get_wishlist;
pub mod post_wishlist;

use axum::{routing::get, Router};

use self::{get_wishlist::get_wishlists, post_wishlist::post_wishlist};

pub fn register_routes(router: Router) -> Router {
    router.route(
        "/wishlists/:user_id",
        get(get_wishlists).post(post_wishlist),
    )
}
