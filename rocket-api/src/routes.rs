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
pub fn get_records_error(state: &State<AppState>) -> Result<Json<Records>, Error> {
    let records = state.service.get_records_error()?;
    let response = Json(Records{records: records});
    Ok(response)
}
