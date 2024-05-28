#![cfg(test)]

use crate::routes::user::service::UserService;

#[test]
pub fn get_user_by_id() {
    let service = UserService::new();

    let user = service.find_user_by_id(10);

    assert_eq!(user.user_id, 10);
}

#[test]
pub fn get_user_list() {
    let service = UserService::new();

    let list = service.find_user_list();

    let mut other = list.clone();
    other.sort_by(|lhs, rhs| lhs.user_id.cmp(&rhs.user_id));

    assert_eq!(list, other);
}
