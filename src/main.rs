#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;

#[get("/", rank = 2)]
fn index() -> Redirect {
    Redirect::to("/index.html")
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index])
        .launch();
}
