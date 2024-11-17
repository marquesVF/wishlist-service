use crate::{
    routes::{
        get_wishlist::{__path_get_wishlist_by_id, __path_get_wishlists_from_user},
        post_wishlist::{WishlistCreation, __path_post_wishlist},
        patch_wishlist::{AddProductToWishlist, __path_add_item_to_wishlist},
    },
    wishlist::{Product, Wishlist},
};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Wishlist service in Rust",
    ),
    paths(
        get_wishlists_from_user,
        post_wishlist,
        add_item_to_wishlist,
        get_wishlist_by_id
    ),
    components(schemas(Wishlist, WishlistCreation, Product, AddProductToWishlist)),
    tags(
        (name = "Wishlist", description = "Manage users wishlists")
    ),
)]
struct ApiDoc;

pub fn register_swagger(router: Router) -> Router {
    router.merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
}
