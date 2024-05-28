/*
 * @Author: plucky
 * @Date: 2023-07-01 17:40:57
 * @LastEditTime: 2023-07-02 21:52:04
 */


#![allow(clippy::new_without_default)]
use std::collections::HashMap;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::{mpsc, Mutex};


type Message = String;
type Sender = mpsc::UnboundedSender<Message>;
// type Receiver = mpsc::UnboundedReceiver<Message>;


 pub struct ChatRoom {
    name: String,
    participants: Mutex<HashMap<String, Sender>>,
}
 impl ChatRoom {
    pub fn new(name: String) -> Self {
        ChatRoom {
            name,
            participants: Mutex::new(HashMap::new()),
        }
    }
   
    pub async fn join(&self, user_name: String, sender: Sender) {
         println!("{} 加入了聊天室 {}", user_name, self.name);
        self.participants.lock().await.insert(user_name, sender);
    }
    pub async fn leave(&self, user_name: String) {
        self.participants.lock().await.remove(&user_name);
        println!("{} 退出了聊天室 {}", user_name, self.name);
    }
    pub async fn send_message(&self, sender: String, message: String) {
        let participants = self.participants.lock().await;
        let message = format!("{}: {}", sender, message);
        participants.par_iter().for_each(|(_name, _sender)| {
            let _= _sender.send(message.clone());
            
        });
        
    }
}


pub struct ChatRoomManager {
    rooms: Mutex<HashMap<String, ChatRoom>>,
}

impl ChatRoomManager {
    pub fn new() -> Self {
        ChatRoomManager {
            rooms: Mutex::new(HashMap::new()),
        }
    }

    pub async fn create_room(&self, room_name: String) {
        let mut rooms = self.rooms.lock().await;
        if !rooms.contains_key(&room_name) {
            println!("聊天室 {} 创建成功", room_name);
            rooms.insert(room_name.clone(), ChatRoom::new(room_name));
        } else {
            println!("聊天室 {} 已存在", room_name);
        }
    }

    pub async fn delete_room(&self, room_name: String) {
        let mut rooms = self.rooms.lock().await;
        if let Some(_r) = rooms.remove(&room_name) {
            println!("聊天室 {} 删除成功", room_name);
        } else {
            println!("聊天室 {} 不存在", room_name);
        }
    }

    pub async fn join_room(&self, room_name: String, user_name: String, sender: Sender) {
        let rooms = self.rooms.lock().await;
        if let Some(room) = rooms.get(&room_name) {
            room.join(user_name, sender).await;
        } else {
            println!("聊天室 {} 不存在", room_name);
        }
    }

    pub async fn leave_room(&self, room_name: String, user_name: String) {
        let rooms = self.rooms.lock().await;
        if let Some(room) = rooms.get(&room_name) {
            room.leave(user_name).await;
        } else {
            println!("聊天室 {} 不存在", room_name);
        }
    }

    pub async fn send_message(&self, room_name: String, user_name: String, message: String) {
        let rooms = self.rooms.lock().await;
        if let Some(room) = rooms.get(&room_name) {
            room.send_message(user_name, message).await;
        } else {
            println!("聊天室 {} 不存在", room_name);
        }
    }
}

#[tokio::main]
async fn main() {
    let chat_room_manager = ChatRoomManager::new();
    chat_room_manager.create_room("rust".to_string()).await;
    chat_room_manager.create_room("golang".to_string()).await;

    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    chat_room_manager.join_room("rust".to_string(), "anger".to_string(), tx).await;

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("anger收到消息: {}", message);
        }
    });

    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    chat_room_manager.join_room("rust".to_string(), "plucky".to_string(), tx).await;

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("plucky收到消息: {}", message);
        }
    });

    chat_room_manager.send_message("rust".to_string(), "abc".to_string(), "hello".to_string()).await;
    chat_room_manager.send_message("rust".to_string(), "abc".to_string(), "world".to_string()).await;

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

}