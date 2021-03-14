use serde::Serialize;
use crate::rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct ProtectedStatus {
    status: String
}

#[get("/protected")]
pub fn status() -> Json<ProtectedStatus> {
    Json(ProtectedStatus{
        status: String::from("This response is from a protected endpoint")
    })
}
