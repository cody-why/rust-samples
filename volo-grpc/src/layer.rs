/*
 * @Author: plucky
 * @Date: 2022-09-09 00:11:13
 * @LastEditTime: 2023-11-05 12:42:06
 * @Description: 
 */

use std::fmt::Debug;

#[derive(Clone)]
pub struct LogService<S>(S);


#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: Send + 'static + Debug,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    Cx: Send + 'static,
    <S as volo::Service<Cx, Req>>::Error: Debug,
    <S as volo::Service<Cx, Req>>::Response: Debug
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        // tracing::info!("Received request {:?}", &req);
        let resp = self.0.call(cx, req).await;
        tracing::info!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}


pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}