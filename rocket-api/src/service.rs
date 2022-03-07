use crate::{entities::Record, error::Error};

#[derive(Debug)]
pub struct Service {}

impl Service {
    pub fn new() -> Service {
        Service {}
    }

    pub fn get_records(&self) -> Result<Vec<Record>, Error> {
        let mut records = Vec::new();
        records.push(Record{
            message: "Hello world".to_string(),
        });
        records.push(Record{
            message: "This is an example".to_string(),
        });

        Ok(records)
    }

    pub fn get_records_error(&self) -> Result<Vec<Record>, Error> {
        Err(Error::Internal("internal server error from get_records_error".to_string()))
    }
}