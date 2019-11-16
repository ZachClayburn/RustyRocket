#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::sync::atomic::

use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket::request::{Self, Form, FormError, FromRequest, FormDataError, FormParseError};
use rocket::http::Cookies;
use rocket::{Outcome, Request};

#[get("/", rank = 2)]
fn index() -> Redirect {
    Redirect::to("/index.html")
}

struct Visitor{
    visitor_id: usize,
}

impl<'a, 'r>FromRequest for Visitor {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        request.cookies()
            .get("visitor_id")
            .o

    }
}

#[derive(Debug, FromForm)]
struct Login {
    username: String,
    password: String,
}

#[post("/login", data = "<login>")]
fn login(login: Result<Form<Login>, FormError>) -> String {
    match login {
        Ok(login) => {format!("{:?}", &*login)},
        Err(err) => {format!("{:?}", err)},
    }
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index])
        .mount("/users", routes![login])
        .launch();
}
