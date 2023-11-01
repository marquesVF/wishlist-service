use crate::routes::{
    get_wishlist::__path_get_wishlists_from_user,
    post_wishlist::{CreateWishlist, __path_post_wishlist},
    put_wishlist::{AddProductToWishlist, __path_put_item_in_wishlists},
};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use wishlist::{Product, Wishlist};

#[derive(OpenApi)]
#[openapi(
    paths(get_wishlists_from_user, post_wishlist, put_item_in_wishlists),
    components(schemas(Wishlist, CreateWishlist, Product, AddProductToWishlist))
)]
struct ApiDoc;

pub fn register_swagger(router: Router) -> Router {
    router.merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
}
