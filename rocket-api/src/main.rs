#[macro_use] extern crate rocket;

mod entities;
mod error;
mod routes;
mod service;
mod state;

use routes::{get_records, get_internal_error, get_bad_request_error};

#[launch]
async fn rocket() -> _ {
    // setup app state
    let service = service::Service::new();
    let app_state = state::AppState::new(service);

    rocket::build()
    .mount("/", routes![get_records, get_internal_error, get_bad_request_error])
    .manage(app_state)
}
