use crate::ressources::Version::Version;
use rocket::serde::json::Json;

//#[openapi]
#[get("/version")]
pub fn version() -> Json<Version> {
    Json(Version {
        code: "v1".to_string(),
        description: "No data should be lost, ever.".to_string(),
    })
}
