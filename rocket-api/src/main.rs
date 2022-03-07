#[macro_use] extern crate rocket;

mod entities;
mod error;
mod routes;
mod service;
mod state;

#[launch]
async fn rocket() -> _ {
    // setup app state
    let service = service::Service::new();
    let app_state = state::AppState::new(service);

    rocket::build()
    .mount("/", routes![routes::get_records, routes::get_records_error])
    .manage(app_state)
}
