/*
 * @Author: plucky
 * @Date: 2022-11-10 23:22:20
 * @LastEditTime: 2022-11-11 00:22:09
 * @Description: 
 */

use dashmap::DashMap;
use tokio::sync::broadcast;

pub struct AppState {
    pub group_list: DashMap<String, broadcast::Sender<String>>,
    pub nc: nats::asynk::Connection,
    // pub users: DashMap<String, String>,
    
}

impl AppState {
    pub fn new(nc: nats::asynk::Connection) -> Self {
        Self {
            group_list: DashMap::new(),
            nc,
        }
    }

    /// get sender or create new one
    pub fn setup_sender(&self, group_id: &str) -> broadcast::Sender<String> {
        // let mut group_list = state.group_list.lock().await;
        let group_id = group_id.to_string();
    
        self.group_list
            .entry(group_id)
            .or_insert_with(|| broadcast::channel(100).0)
            .clone()
    }

    pub fn get_group_count(&self, group_id: &str) -> usize{
        // Locking behaviour by get
        if let Some(group) = self.group_list.get(group_id) {
            return group.receiver_count();
        }
        0
    }

    pub fn remove_group(&self, group_id: &str) -> bool{
        // 用户断开时自动删除channel,所以只有1个用户时，删除group
        if self.get_group_count(group_id) == 0 {
            self.group_list.remove(group_id);
            return true;
        }
        false
    }

}