#[get("/")]
pub fn index() -> &'static str {
    "No data should be lost, ever."
}
