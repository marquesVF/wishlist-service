use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};

use crate::wishlists::Wishlist;

pub fn register_routes(router: Router) -> Router {
    router.route("/wishlists/:user_id", get(get_wishlists))
}

#[utoipa::path(
    get,
    path = "/wishlists/:user_id",
    tag = "Wishlists",
    responses(
        (status = 200, body = Wishlist, description = "Returns an user's wishlists"),
        (status = 404, description = "The user has no wishlists"),
    )
)]
async fn get_wishlists(Path(user_id): Path<String>) -> impl IntoResponse {
    let wishlist = Wishlist {
        name: "default".to_string(),
        user_id,
    };

    Json(vec![wishlist])
}
