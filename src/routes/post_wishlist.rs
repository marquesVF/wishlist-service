use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use utoipa::ToSchema;

use crate::provider::wishlist_provider;

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct CreateWishlist {
    pub name: String,
}

#[utoipa::path(
    post,
    path = "/wishlists/from_user/{user_id}",
    tag = "Wishlist",
    request_body = CreateWishlist,
    responses(
        (status = 201, body = Wishlist, description = "Wishlist was created"),
        (status = 404, description = "The user has no wishlists"),
    )
)]
pub async fn post_wishlist(
    Path(user_id): Path<String>,
    Json(input): Json<CreateWishlist>,
) -> impl IntoResponse {
    let wishlists = wishlist_provider()
        .await
        .create_wishlist(input.name, user_id)
        .await;

    (StatusCode::CREATED, Json(wishlists))
}
