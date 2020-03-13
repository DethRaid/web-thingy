//! API keys are uwu
//!
//! They can only be generated through the admin panel

use std::sync::Mutex;
use oxide_auth::primitives::prelude::*;
use oxide_auth::frontends::rocket::{OAuthRequest, OAuthFailure};
use rocket::{State, Response, Data};
use oxide_auth::frontends::simple::endpoint::FnSolicitor;
use rocket::response::Responder;
use std::io;

struct ApiKey(String);

fn is_valid(key: &str) -> bool {
    // TODO: A real check for OAuth or something I guess?
    unimplemented!();
}
