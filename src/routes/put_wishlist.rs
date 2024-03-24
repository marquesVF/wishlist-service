use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use utoipa::ToSchema;

use crate::{
    db_provider::wishlists::add_product_to_wishlist, state::ServerState, wishlist::Wishlist,
};

use super::RouteResponse;

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct AddProductToWishlist {
    pub product_sku: String,
}

#[utoipa::path(
    put,
    path = "/wishlists/{wishlist_id}",
    tag = "Wishlist",
    responses(
        (status = 201, body = Vec<Wishlist>, description = "Returns the updated user's wishlists"),
        (status = 404, description = "The user has no wishlists"),
    )
)]
pub async fn put_item_in_wishlist(
    State(state): State<ServerState>,
    Path(wishlist_id): Path<i32>,
    Json(add_product): Json<AddProductToWishlist>,
) -> RouteResponse<Wishlist> {
    let wishlist = add_product_to_wishlist(&wishlist_id, &add_product.product_sku, &state.db_pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(wishlist)))
}
