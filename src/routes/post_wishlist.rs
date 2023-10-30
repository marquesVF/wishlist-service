use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use utoipa::ToSchema;

use super::get_wishlist::Wishlist;

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct CreateWishlist {
    pub name: String,
}

#[utoipa::path(
    post,
    path = "/wishlists/{user_id}",
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
    let wishlist = Wishlist {
        name: input.name,
        user_id,
    };

    (StatusCode::CREATED, Json(vec![wishlist]))
}
