use serde::{Deserialize, Serialize};
use super::super::vo::hello::Hello;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHelloResponse {
    pub data: Hello,
}