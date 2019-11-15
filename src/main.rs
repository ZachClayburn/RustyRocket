#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket::request::{Form, FormError, FormDataError, FormParseError};

#[get("/", rank = 2)]
fn index() -> Redirect {
    Redirect::to("/index.html")
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
