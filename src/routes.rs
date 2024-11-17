pub mod get_wishlist;
pub mod patch_wishlist;
pub mod post_wishlist;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::state::ServerState;

use self::{
    get_wishlist::{get_wishlist_by_id, get_wishlists_from_user},
    patch_wishlist::add_item_to_wishlist,
    post_wishlist::post_wishlist,
};

type RouteResponse<T> = Result<(StatusCode, Json<T>), (StatusCode, String)>;

pub fn register_routes(state: ServerState) -> Router {
    Router::new()
        .route(
            "/wishlists/from_user/:user_id",
            get(get_wishlists_from_user),
        )
        .route("/wishlists", post(post_wishlist))
        .route(
            "/wishlists/:wishlist_id",
            get(get_wishlist_by_id).patch(add_item_to_wishlist),
        )
        .with_state(state)
}
