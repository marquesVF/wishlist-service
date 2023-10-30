use super::wishlists::Wishlist;
use crate::routes::__path_get_wishlists;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
        paths(
            get_wishlists,
        ),
        components(
            schemas(Wishlist)
        ),
    )]
struct ApiDoc;

pub fn register_swagger(router: Router) -> Router {
    router.merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
}
