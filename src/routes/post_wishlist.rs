use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{db_provider::wishlists::create_wishlist, state::ServerState, wishlist::Wishlist};

use super::RouteResponse;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WishlistCreation {
    pub name: String,
    pub user_id: String,
    pub product_sku: String,
}

#[utoipa::path(
    post,
    path = "/wishlists",
    tag = "Wishlist",
    request_body = WishlistCreation,
    responses(
        (status = 201, body = Wishlist, description = "Wishlist was created"),
        (status = 404, description = "The user has no wishlists"),
    )
)]
pub async fn post_wishlist(
    State(state): State<ServerState>,
    Json(input): Json<WishlistCreation>,
) -> RouteResponse<Wishlist> {
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
