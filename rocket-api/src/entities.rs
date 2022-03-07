use rocket::serde::{Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Record {
    pub message: String,
}
