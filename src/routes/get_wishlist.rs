use axum::{extract::Path, response::IntoResponse, Json};
use data_provider::wishlists::{get_user_wishlists, get_wishlist};

#[utoipa::path(
    get,
    path = "/wishlists/from_user/{user_id}",
    tag = "Wishlist",
    responses(
        (status = 200, body = Vec<Wishlist>, description = "Returns an user's wishlists"),
        (status = 404, description = "The user has no wishlists"),
    )
)]
pub async fn get_wishlists_from_user(Path(user_id): Path<String>) -> impl IntoResponse {
    let wishlist = get_user_wishlists(user_id.as_str()).await;

    Json(wishlist)
}

#[utoipa::path(
    get,
    path = "/wishlists/{wishlist_id}",
    tag = "Wishlist",
    responses(
        (status = 200, body = Vec<Wishlist>, description = "Returns an user's wishlists"),
        (status = 404, description = "The user has no wishlists"),
    )
)]
pub async fn get_wishlist_by_id(Path(wishlist_id): Path<i32>) -> impl IntoResponse {
    let wishlist = get_wishlist(&wishlist_id).await;

    Json(wishlist)
}
