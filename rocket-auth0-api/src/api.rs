use serde::Serialize;
use crate::rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct ApiStatus {
    status: String
}

#[get("/")]
pub fn status() -> Json<ApiStatus> {
    Json(ApiStatus{
        status: String::from("Up and running!")
    })
}
