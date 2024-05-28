use super::dtos::create_hello_request::CreateHelloRequest;
use super::dtos::create_hello_response::CreateHelloResponse;
use super::dtos::get_hello_response::GetHelloResponse;
use super::dtos::list_hello_response::ListHelloResponse;
use super::dtos::update_hello_request::UpdateHelloRequest;
use super::dtos::update_hello_response::UpdateHelloResponse;
use super::dtos::delete_hello_response::DeleteHelloResponse;

use super::vo::hello::Hello;

pub struct HelloService {}

impl HelloService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_one(&self, _dto: CreateHelloRequest) -> CreateHelloResponse {
        // create something...
        CreateHelloResponse { id: 1 }
    }

    pub fn update_one(&self, _id: i32, _dto: UpdateHelloRequest) -> UpdateHelloResponse {
        // update something...
        UpdateHelloResponse { }
    }

    pub fn delete_one(&self, _id: i32) -> DeleteHelloResponse {
        // delete something...
        DeleteHelloResponse { }
    }

    pub fn find_one(&self, id: i32) -> GetHelloResponse {
        // find something...
        GetHelloResponse { 
            data: Hello { 
                id,
            } 
        }
    }

    pub fn find_all(&self) -> ListHelloResponse {
        // find something...
        ListHelloResponse { 
            list: vec![
                Hello { 
                    id: 1
                } 
            ] 
        }
    }
}
