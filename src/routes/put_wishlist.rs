use axum::{extract::Path, response::IntoResponse, Json};
use utoipa::ToSchema;

use crate::provider::wishlist_provider;

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct AddProductToWishlist {
    pub product_sku: String,
}

#[utoipa::path(
    put,
    path = "/wishlists/{wishlist_id}",
    tag = "Wishlist",
    responses(
        (status = 200, body = Vec<Wishlist>, description = "Returns the updated user's wishlists"),
        (status = 404, description = "The user has no wishlists"),
    )
)]
pub async fn put_item_in_wishlists(
    Path(wishlist_id): Path<String>,
    Json(add_product): Json<AddProductToWishlist>,
) -> impl IntoResponse {
    let wishlists = wishlist_provider()
        .await
        .add_product_to_wishlist(wishlist_id, add_product.product_sku)
        .await;

    Json(wishlists)
}
