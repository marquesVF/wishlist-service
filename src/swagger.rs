use crate::{
    routes::{
        get_wishlist::{__path_get_wishlist_by_id, __path_get_wishlists_from_user},
        post_wishlist::{WishlistCreation, __path_post_wishlist},
        put_wishlist::{AddProductToWishlist, __path_put_item_in_wishlist},
    },
    wishlist::{Product, Wishlist},
};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_wishlists_from_user,
        post_wishlist,
        put_item_in_wishlist,
        get_wishlist_by_id
    ),
    components(schemas(Wishlist, WishlistCreation, Product, AddProductToWishlist)),
    tags(
        (name = "Wishlist", description = "Manage users wishlists")
    )
)]
struct ApiDoc;

pub fn register_swagger(router: Router) -> Router {
    router.merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
}
