/*
 * @Author: plucky
 * @Date: 2023-07-01 17:40:57
 * @LastEditTime: 2023-07-03 07:52:13
 */


#![allow(clippy::new_without_default)]
use dashmap::DashMap;
use tokio::sync::{broadcast, oneshot};


type Message = String;
type Sender = broadcast::Sender<Message>;
// type Receiver = broadcast::Receiver<Message>;


 pub struct ChatRoom {
    name: String,
    sender: Sender,
    participants: DashMap<String, oneshot::Sender<()>>,
}
 impl ChatRoom {
    pub fn new(name: String) -> Self {
        let (sender, _) = broadcast::channel(100);
        ChatRoom {
            name,
            sender,
            participants: DashMap::new(),
        }
    }
   
    pub async fn join(&self, user_name: String, sender: Sender) {
        println!("{} 加入了聊天室 {}", user_name, self.name);
        let mut receiver = self.sender.subscribe();
        let chat_room_name = self.name.clone();
        if self.participants.contains_key(&user_name) {
            println!("{} 已经在聊天室 {}", user_name, chat_room_name);
            return;
        }
        let (tx, rx) = oneshot::channel::<()>();
        self.participants.insert(user_name.clone(), tx);
        
        tokio::spawn(async move {
            tokio::select! {
                _ = async {
                    loop {
                        let message = receiver.recv().await;
                        match message{
                            Ok(message) => {
                                if let Err(e) = sender.send(message) {
                                    println!("Send failed: {e}, {} 退出了聊天室 {}", user_name, chat_room_name);
                                    break;
                                }
                            }
                            Err(_e) => {
                                println!("关闭了聊天室, {} 退出了聊天室 {}", user_name, chat_room_name);
                                break;
                            }
                        }
                    
                    }
                }=> {},
                _ = rx => {
                    println!("{} 退出了聊天室 {}", user_name, chat_room_name);
                }
                
            }
        });
        
    }

    pub async fn leave(&self, user_name: String) {
        self.participants.remove(&user_name);
    }

    pub async fn send_message(&self, sender: String, message: String) {
        println!("{} 发送了消息 {}", sender, message);
        let _ = self.sender.send(message);
        
    }
}


pub struct ChatRoomManager {
    rooms: DashMap<String, ChatRoom>,
}


impl ChatRoomManager {
    pub fn new() -> Self {
        ChatRoomManager {
            rooms: DashMap::new(),
        }
    }

    pub async fn create_room(&self, room_name: String) {
        // let mut rooms = self.rooms.lock().await;
        if !self.rooms.contains_key(&room_name) {
            println!("聊天室 {} 创建成功", room_name);
            self.rooms.insert(room_name.clone(), ChatRoom::new(room_name));
        } else {
            println!("聊天室 {} 已存在", room_name);
        }
    }

    pub async fn delete_room(&self, room_name: String) {
        // let mut rooms = self.rooms.lock().await;
        if let Some(_r) = self.rooms.remove(&room_name) {
            println!("聊天室 {} 删除成功", room_name);
        } else {
            println!("聊天室 {} 不存在", room_name);
        }
    }

    pub async fn join_room(&self, room_name: String, user_name: String, sender: Sender) {
        // let rooms = self.rooms.lock().await;
        if let Some(room) = self.rooms.get(&room_name) {
            room.join(user_name, sender).await;
        } else {
            println!("聊天室 {} 不存在", room_name);
        }
    }

    pub async fn leave_room(&self, room_name: String, user_name: String) {
        // let rooms = self.rooms.lock().await;
        if let Some(room) = self.rooms.get(&room_name) {
            room.leave(user_name).await;
        } else {
            println!("聊天室 {} 不存在", room_name);
        }
    }

    pub async fn send_message(&self, room_name: String, user_name: String, message: String) {
        // let rooms = self.rooms.lock().await;
        if let Some(room) = self.rooms.get(&room_name) {
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

    let (tx, mut rx) = broadcast::channel::<Message>(100);
    chat_room_manager.join_room("rust".to_string(), "anger".to_string(), tx.clone()).await;

    tokio::spawn(async move {
        loop {
            let message = rx.recv().await;
            match message {
                Ok(message) => {
                    println!("anger收到消息: {}", message);
                }
                Err(_) => {
                    println!("退出了聊天室");
                    break;
                }
            }
        }
    });
    {
        let (tx, _) = broadcast::channel::<Message>(100);
        chat_room_manager.join_room("rust".to_string(), "plucky".to_string(), tx.clone()).await;
    }
    // chat_room_manager.leave_room("rust".to_string(), "anger".to_string()).await;
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
   

    chat_room_manager.send_message("rust".to_string(), "abc".to_string(), "hello".to_string()).await;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    chat_room_manager.delete_room("rust".to_string()).await;

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

}