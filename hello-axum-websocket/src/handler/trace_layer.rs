/*
 * @Author: plucky
 * @Date: 2022-11-01 09:22:30
 * @LastEditTime: 2022-11-01 10:52:44
 * @Description: 
 */

use std::{time::Duration};

use axum::{http::{Request, Response, HeaderMap}, body::{Bytes, Body}, Router};
use tower_http::{trace::TraceLayer, classify::{ServerErrorsFailureClass}};
use tracing::{debug, Span};

#[allow(dead_code)]
pub fn trace_layer(r: Router) -> Router {
    let layer = TraceLayer::new_for_http()
        .make_span_with(|_request: &Request<Body>| {
            tracing::debug_span!("http-request")
        })
        .on_request(|request: &Request<Body>, _span: &Span| {
            debug!("started {} {}", request.method(), request.uri().path())
        })
        .on_response(|_response: &Response<_>, latency: Duration, _span: &Span| {
            debug!("response generated in {:?}", latency)
        })
        .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
            debug!("sending {} bytes", chunk.len())
        })
        .on_eos(|_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
            debug!("stream closed after {:?}", stream_duration)
        })
        .on_failure(|_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
            debug!("something went wrong")
        });

    r.layer(layer)
    
    
}


