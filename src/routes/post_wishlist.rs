use axum::{http::StatusCode, response::IntoResponse, Json};
use sqlite_provider::create_wishlist;
use utoipa::ToSchema;

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct CreateWishlist {
    pub name: String,
    pub user_id: String,
    pub product_sku: String,
}

#[utoipa::path(
    post,
    path = "/wishlists",
    tag = "Wishlist",
    request_body = CreateWishlist,
    responses(
        (status = 201, body = Wishlist, description = "Wishlist was created"),
        (status = 404, description = "The user has no wishlists"),
    )
)]
pub async fn post_wishlist(Json(input): Json<CreateWishlist>) -> impl IntoResponse {
    let wishlists = create_wishlist(input.name, input.product_sku, input.user_id).await;

    (StatusCode::CREATED, Json(wishlists))
}
