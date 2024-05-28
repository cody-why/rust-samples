/*
 * @Author: plucky
 * @Date: 2022-10-24 16:47:51
 * @LastEditTime: 2023-11-05 12:52:22
 * @Description: 
 */
#![feature(type_alias_impl_trait)]
pub mod layer;

use std::collections::HashMap;
use volo_gen::volo::example::{GetItemRequest,GetItemResponse, Item};

pub struct S;

impl volo_gen::volo::example::ItemService for S {
    // 这部分是我们需要增加的代码
    async fn get_item(
        &self,
        _req: GetItemRequest,
    ) -> core::result::Result<GetItemResponse, volo_thrift::AnyhowError>{
        tracing::info!("Received request {:?}", &_req);
        let mut v =  HashMap::new();
        v.insert("name".into(), "jack".into());
        
        Ok(GetItemResponse {
            item: Item {
                id: _req.id,
                title: "test".into(),
                content: "hello".into(),
                extra : Some(v),
            }

        })
        // Ok(Default::default())

    }
}
