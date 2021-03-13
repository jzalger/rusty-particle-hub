#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib; 
extern crate serde;
#[macro_use] extern crate serde_derive;

mod models;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
// use serde_json::value::Index;

#[derive(Serialize)]
struct Context {}

#[get("/")]
fn root() -> Template {
    // TODO: add CSRF token to context
    let context = Context{};
    Template::render("particlehub", &context)
}

#[get("/get-devices")]
fn get_devices() {
    let cloud = models::ParticleCloud::new("abc123");
}


fn main() {
    rocket::ignite()
    .attach(Template::fairing())
    .mount("/", routes![root])
    .mount("/static", StaticFiles::from("static"))
    .launch();
}
