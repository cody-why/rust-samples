#![cfg(test)]

use crate::routes::root::service::RootService;

#[test]
pub fn get_health_check() {
    let service = RootService::new();

    let response = service.get_health();

    assert_eq!(response.server_ok, true);
}
