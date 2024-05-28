use super::dtos::health_response::HealthReponse;

pub struct RootService {}

impl RootService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_health(&self) -> HealthReponse {
        let server_ok = true;

        HealthReponse { server_ok }
    }
}
