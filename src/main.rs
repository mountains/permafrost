#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit="256"]
#[macro_use]
extern crate rocket;
extern crate serde;
extern crate serde_json;
pub mod cors;
pub mod models;
pub mod routes;
pub mod ressources;

fn main() {
    println!("No data should be lost, ever.");
    rocket::ignite()
        //.manage(connection::connect())
        .attach(cors::CorsFairing)
        .mount("/", routes![
            routes::index::index,
        ])
        .launch();
}
