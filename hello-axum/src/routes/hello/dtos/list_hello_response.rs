use serde::{Deserialize, Serialize};
use super::super::vo::hello::Hello;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHelloResponse {
    pub list: Vec<Hello>,
}