use rocket::Route;
use crate::api;
use crate::protected;

pub fn get_routes() -> Vec<Route> {
    routes![
        api::status,
        protected::status,
    ]
}
