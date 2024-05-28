/*
 * @Author: plucky
 * @Date: 2023-11-05 10:51:37
 * @LastEditTime: 2023-11-05 12:44:10
 */

pub mod layer;

use std::collections::HashMap;

use volo_gen::volo::example::{GetItemResponse, Item};

pub struct S;

impl volo_gen::volo::example::ItemService for S {
    async fn get_item(
        &self,
        _req: volo_grpc::Request<volo_gen::volo::example::GetItemRequest>,
    ) -> std::result::Result<::volo_grpc::Response<volo_gen::volo::example::GetItemResponse>, volo_grpc::Status>
    {
        tracing::info!("Received request {:?}", &_req);

        let mut v =  HashMap::new();
        v.insert("name".into(), "jack".into());
        
        let resp = GetItemResponse{
            item:Some(Item {
                id: _req.get_ref().id,
                title: "test".into(),
                content: "hello".into(),
                extra : v,
            })
        };
        Ok(volo_grpc::Response::new(resp))

        // Ok(volo_grpc::Response::new(Default::default()))
    }
}
