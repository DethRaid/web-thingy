#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{routes, get};

mod admin;
mod auth;
mod images;

#[get("/")]
fn index() -> &'static str {
    "Hello world, stranger!"
}

#[get("/<name>")]
fn name(name: String) -> String {
    format!("Hello, world, {}!", name)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, name])
        .mount("/images", routes![images::get_image])
        .launch();
}