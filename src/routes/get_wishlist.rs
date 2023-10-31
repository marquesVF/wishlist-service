use axum::{extract::Path, response::IntoResponse, Json};
use sqlite_provider::find_user_wishlists;

#[utoipa::path(
    get,
    path = "/wishlists/{user_id}",
    tag = "Wishlist",
    responses(
        (status = 200, body = Vec<Wishlist>, description = "Returns an user's wishlists"),
        (status = 404, description = "The user has no wishlists"),
    )
)]
pub async fn get_wishlists(Path(user_id): Path<String>) -> impl IntoResponse {
    let wishlist = find_user_wishlists(&user_id).await;

    Json(vec![wishlist])
}
