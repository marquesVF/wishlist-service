use crate::routes::{
    get_wishlist::__path_get_wishlists,
    post_wishlist::{CreateWishlist, __path_post_wishlist},
};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use wishlist::{Product, Wishlist};

#[derive(OpenApi)]
#[openapi(
    paths(get_wishlists, post_wishlist),
    components(schemas(Wishlist, CreateWishlist, Product))
)]
struct ApiDoc;

pub fn register_swagger(router: Router) -> Router {
    router.merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
}
