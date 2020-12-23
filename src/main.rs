#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "256"]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_okapi;

extern crate serde;
extern crate serde_json;
use rocket_okapi::swagger_ui::*;
use dotenv::dotenv;
pub mod cors;
pub mod models;
pub mod requests;
pub mod ressources;
pub mod routes;
pub mod connection;

fn main() {
    println!("No data should be lost, ever.");
    dotenv().ok();
    rocket::ignite()
        .manage(connection::connect())
        .attach(cors::CorsFairing)
        .mount(
            "/",
            routes_with_openapi![
                routes::index::index,
                routes::version::version,
                routes::v1::git::commits::commits,
                routes::v1::git::branches::branches,
            ],
        )
        .mount(
            "/dev/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch();
}
