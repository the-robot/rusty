use crate::service::Service;

#[derive(Debug)]
pub struct AppState {
    pub service: Service,
}

impl AppState {
    pub fn new(service: Service) -> AppState {
        AppState { service }
    }
}
