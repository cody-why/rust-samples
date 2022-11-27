/*
 * @Author: plucky
 * @Date: 2022-10-24 16:47:51
 * @LastEditTime: 2022-11-24 21:30:53
 * @Description: 
 */
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]
pub mod layer;

use std::{collections::HashMap};
use volo_gen::volo::example::{GetItemRequest,GetItemResponse, Item};

pub struct S;

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
    // 这部分是我们需要增加的代码
    async fn get_item(
        &self,
        _req: GetItemRequest,
    ) -> core::result::Result<GetItemResponse, volo_thrift::AnyhowError>{
        println!("get_item called with {:?}", _req);
        // Ok(Default::default())
        let mut v =  HashMap::new();
        v.insert("name".to_string(), "jack".to_string());
        
        Ok(GetItemResponse {
            item: Item {
                id: _req.id,
                title: "test".to_string(),
                content: "hello".to_string(),
                extra : Some(v),
            }

        })
    }
}
