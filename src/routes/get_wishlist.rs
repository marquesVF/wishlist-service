use axum::{extract::Path, response::IntoResponse, Json};
use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
pub struct Wishlist {
    pub user_id: String,
    pub name: String,
}

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
    let wishlist = Wishlist {
        name: "default".to_string(),
        user_id,
    };

    Json(vec![wishlist])
}
