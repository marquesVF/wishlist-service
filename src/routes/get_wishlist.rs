use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    db_provider::wishlists::{get_user_wishlists, get_wishlist},
    state::ServerState,
    wishlist::Wishlist,
};

use super::RouteResponse;

#[utoipa::path(
    get,
    path = "/wishlists/from_user/{user_id}",
    tag = "Wishlist",
    responses(
        (status = 200, body = Vec<Wishlist>, description = "Returns an user's wishlists"),
        (status = 404, description = "The user has no wishlists"),
    )
)]
pub async fn get_wishlists_from_user(
    State(state): State<ServerState>,
    Path(user_id): Path<String>,
) -> Result<Json<Vec<Wishlist>>, (StatusCode, String)> {
    let wishlist = get_user_wishlists(user_id.as_str(), &state.db_pool)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    if wishlist.len() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            format!("no wishlists found for user '{}'", user_id),
        ));
    }

    Ok(Json(wishlist))
}

#[utoipa::path(
    get,
    path = "/wishlists/{id}",
    tag = "Wishlist",
    responses(
        (status = 200, body = Vec<Wishlist>, description = "Returns an user's wishlists"),
        (status = 404, description = "The user has no wishlists"),
    )
)]
pub async fn get_wishlist_by_id(
    State(state): State<ServerState>,
    Path(id): Path<i32>,
) -> RouteResponse<Wishlist> {
    let wishlist = get_wishlist(&id, &state.db_pool).await.map_err(|e| {
        let mut msg = e.to_string();

        if msg.contains("no rows returned") {
            msg = format!("wishlist {} doesn't exist", id);
        }

        (StatusCode::NOT_FOUND, msg)
    })?;

    Ok((StatusCode::OK, Json(wishlist)))
}
