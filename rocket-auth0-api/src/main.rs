#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

mod api;

fn main() {
    rocket::ignite().mount("/", routes![api::status]).launch();
}
