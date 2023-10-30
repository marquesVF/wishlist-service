mod routes;
mod swagger;
mod wishlists;

use rocket::launch;
use routes::register_routes;
use swagger::register_swagger;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let rocket = register_routes(rocket);

    register_swagger(rocket)
}
