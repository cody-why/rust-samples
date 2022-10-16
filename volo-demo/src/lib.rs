/***
 * @Author: plucky
 * @Date: 2022-09-08 22:54:31
 * @LastEditTime: 2022-09-09 00:28:35
 * @Description: 
 */

#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

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
    ) -> core::result::Result<GetItemResponse, pilota::AnyhowError>
    {
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
