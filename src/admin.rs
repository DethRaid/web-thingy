//! Various administrator functionality
//!
//! First use case will be generating API keys. The auth module will provide a way for an authenticated user to generate
//! a new API key. The key will be stored in some database/text file, such that requests to upload, change, or delete an
//! image can check the provided API key against that file

use rocket::get;
use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::templates::{Template, handlebars};
use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};
use std::collections::HashMap;

#[get("/")]
pub fn index_admin(admin: AdminUser) -> Template {
    let mut context = HashMap::new();
    context.insert("test", "test");
    Template::render("admin/index_admin", &context)
}

#[get("/")]
pub fn index_unauth() -> Template {
    let mut context = HashMap::new();
    context.insert("test", "test");
    Template::render("admin/index", &context)
}
