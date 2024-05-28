/*
 * @Author: plucky
 * @Date: 2022-11-17 22:32:40
 * @LastEditTime: 2023-11-03 17:56:45
 * @Description: 
 */

use ice_rs::communicator::Communicator;
use std::collections::HashMap;
use async_trait::async_trait;

use ice_rs_demo::gen::demo::{Contact, ContactDBServer, ContactDBI, NumberType};

struct ContactDB {
    data: HashMap<String, Contact>
}

#[async_trait]
impl ContactDBI for ContactDB {
    async fn add_contact(&mut self, name: &String, num_type: Option<NumberType>, number: Option<String>, dial_group: Option<i32>, _context: Option<HashMap<String, String>>) {        
        let contact = Contact {
            name: name.clone(),
            num_type: if let Some(_value) = num_type { num_type } else { Some(NumberType::HOME) },
            number,
            dial_group
        };
        self.data.insert(name.clone(), contact);
    }

    async fn update_contact(&mut self, name: &String, num_type: Option<NumberType>, number: Option<String>, dial_group: Option<i32>, _context: Option<HashMap<String, String>>) {
        if let Some(contact) = self.data.get_mut(name) {
            contact.num_type = if let Some(_value) = num_type { num_type } else { contact.num_type };
            contact.number = if let Some(_value) = number.clone() { number } else { contact.number.clone() };
            contact.dial_group = if let Some(_value) = dial_group { dial_group } else { contact.dial_group };
        }
    }

    async fn query(&mut self, name: &String, _context: Option<HashMap<String, String>>) -> Contact {
        self.data.get(name).unwrap().clone()
    }

    async fn query_number(&mut self, name: &String, _context: Option<HashMap<String, String>>) -> Option<String> {
        self.data.get(name).unwrap().number.clone()
    }

    async fn query_dialgroup(&mut self, name: &String, dial_group: &mut Option<i32>, _context: Option<HashMap<String, String>>) {
        *dial_group = self.data.get(name).unwrap().dial_group;
    }

    async fn shutdown(&mut self, _context: Option<HashMap<String, String>>) {
        // current.adapter.getCommunicator().shutdown()
        println!("impl shutdown");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let comm = Communicator::new().await?;
    let mut adapter = comm.create_object_adapter_with_endpoint("contactdb", "tcp -h localhost -p 10000").await?;

    println!("Listening on 10000");
    let server = ContactDBServer::new(Box::new(ContactDB{data: HashMap::new()}));

    adapter.add("contactdb", Box::new(server));
    adapter.activate().await?;

    // comm.wait_for_shutdown().await?;

    Ok(())
}