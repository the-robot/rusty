use crate::{entities::Record, error::Error, state::AppState};
use rocket::State;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
pub struct Records {
    pub records: Vec<Record>
}

#[get("/")]
pub fn get_records(state: &State<AppState>) -> Result<Json<Records>, Error> {
    let records = state.service.get_records()?;
    let response = Json(Records{records: records});
    Ok(response)
}

#[get("/error")]
pub fn get_internal_error(state: &State<AppState>) -> Result<Json<Records>, Error> {
    let records = state.service.internal_error()?;
    let response = Json(Records{records: records});
    Ok(response)
}

#[get("/bad")]
pub fn get_bad_request_error(state: &State<AppState>) -> Result<Json<Records>, Error> {
    let records = state.service.bad_request_error()?;
    let response = Json(Records{records: records});
    Ok(response)
}