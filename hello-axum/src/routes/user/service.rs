use super::dtos::user::User;

pub struct UserService {}

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find_user_list(&self) -> Vec<User> {
        let user_list = vec![
            User {
                user_id: 1,
                user_name: "test".to_string(),
            },
            User {
                user_id: 2,
                user_name: "test2".to_string(),
            },
        ];

        user_list
    }

    pub fn find_user_by_id(&self, id: i32) -> User {
        let user = User {
            user_id: id,
            user_name: "test".to_string(),
        };

        user
    }
}
