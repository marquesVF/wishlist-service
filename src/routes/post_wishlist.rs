use axum::{extract::State, http::StatusCode, Json};
use data_provider::wishlists::create_wishlist;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use wishlist::Wishlist;

use crate::state::AppState;

#[derive(Serialize, Deserialize, ToSchema)]
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
pub async fn post_wishlist(
    State(state): State<AppState>,
    Json(input): Json<CreateWishlist>,
) -> Result<(StatusCode, Json<Wishlist>), (StatusCode, String)> {
    let wishlist = create_wishlist(
        &input.name,
        &input.user_id,
        &input.product_sku,
        &state.db_pool,
    )
    .await
    .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(wishlist)))
}
