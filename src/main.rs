#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate rocket_okapi;

use crate::dotenv::dotenv;
use rocket_okapi::swagger_ui::*;
use std::env;

pub mod connection;
pub mod cors;
pub mod database;
pub mod models;
pub mod requests;
pub mod responses;
pub mod ressources;
pub mod routes;

#[launch]
fn rocket() -> _ {
    println!("No data should be lost, ever.");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    rocket::build()
        .manage(connection::establish_connection(database_url))
        .attach(cors::CorsFairing)
        .mount(
            "/dev/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/",
            routes_with_openapi![
                routes::index::index,
                routes::version::version,
                routes::v1::git::commits::commits,
                routes::v1::git::branches::branches,
                routes::v1::git::repositories::list_repositories,
                routes::v1::git::repositories::add_repository,
            ],
        )
}
