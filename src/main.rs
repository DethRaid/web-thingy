#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};
use rocket_contrib::templates::Template;

mod admin;
mod auth;
mod images;
mod oauth;

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
        // .mount("/", routes![index, name])
        .mount("/", routes![
             oauth::authorize,
             oauth::authorize_consent,
             oauth::token,
             oauth::protected_resource,
             oauth::refresh,
             ])
        .mount("/images", routes![images::get_image])
        .mount("/admin", routes![admin::index])
        .attach(Template::fairing())
        .manage(MyState::preconfigured())
        .launch();
}
