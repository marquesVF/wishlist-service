use axum::{extract::Path, response::IntoResponse, Json};
use sqlite_provider::{get_user_wishlists, SQLiteProvider};
use wishlist::WishlistProvider;

#[utoipa::path(
    get,
    path = "/wishlists/from_user/{user_id}",
    tag = "Wishlist",
    responses(
        (status = 200, body = Vec<Wishlist>, description = "Returns an user's wishlists"),
        (status = 404, description = "The user has no wishlists"),
    )
)]
pub async fn get_wishlists(Path(user_id): Path<String>) -> impl IntoResponse {
    //
    // let provider = SQLiteProvider {};
    // provider.

    let wishlist = get_user_wishlists(user_id).await;

    Json(wishlist)
}
