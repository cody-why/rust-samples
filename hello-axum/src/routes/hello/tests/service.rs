
#![cfg(test)]

use crate::routes::hello::service::HelloService;

#[test]
pub fn example() {
    let _service = HelloService::new();

    assert_eq!(10, 10);
}
