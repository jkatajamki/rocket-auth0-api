#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket::fairing::AdHoc;
use auth::authsettings::AuthSettings;

mod api;
mod protected;
mod auth;
mod routes;

fn main() {
    let routes_vec = routes::get_routes();

    rocket::ignite()
        .mount("/", routes_vec)
        .attach(AdHoc::on_attach("authsettings", |rocket: rocket::Rocket| {
            let settings = AuthSettings::from_env().expect("Auth0 settings must be configured in env variables!");

            Ok(rocket.manage(settings))
        }))
        .launch();
}
