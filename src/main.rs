#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "256"]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_okapi;

extern crate serde;
extern crate serde_json;
use rocket_okapi::swagger_ui::*;
pub mod cors;
pub mod models;
pub mod ressources;
pub mod routes;

fn main() {
    println!("No data should be lost, ever.");
    rocket::ignite()
        //.manage(connection::connect())
        .attach(cors::CorsFairing)
        .mount(
            "/",
            routes_with_openapi![
                routes::index::index,
                routes::version::version,
                routes::v1::commits::commits,
                routes::v1::branches::branches,
            ],
        )
        .mount(
            "/dev/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: Some("../../openapi.json".to_owned()),
                urls: None,
            }),
        )
        .launch();
}
