use axum::{extract::Path, response::IntoResponse, Json};
use data_provider::wishlists::add_product_to_wishlist;
use utoipa::ToSchema;

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
    Path(wishlist_id): Path<i32>,
    Json(add_product): Json<AddProductToWishlist>,
) -> impl IntoResponse {
    let wishlists = add_product_to_wishlist(&wishlist_id, &add_product.product_sku).await;

    Json(wishlists)
}
