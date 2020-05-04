/*
mod dhi;
use dhi::{DHIRequest, DHIResponse};

mod errors;
use errors::AppError;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use hyper::{Body, Request, Response, StatusCode};
use serde_json::Value;
use std::net::SocketAddr;

use serde_xml_rs::from_reader;

use async_std::net::TcpStream;
use async_std::prelude::*;

/// Asynchronously exchange data with DHI host
async fn talk_to_dhi_host(msg: String) -> Result<DHIResponse, AppError> {
    let mut s = TcpStream::connect("10.217.13.27:10304").await?;

    s.write_all(&msg.as_bytes()).await?;
    println!("{}", msg);

    let mut buffer = [0; 8192];
    s.read(&mut buffer).await?;

    // The first 5 bytes are message length
    let response: DHIResponse = from_reader(&buffer[5..])?;
    Ok(response)
}

async fn serve_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Got request at {:?}", req.uri());
    println!("Headers: {:?}", req.headers());

    // TODO: routing GET/POST requests

    let body = hyper::body::to_bytes(req.into_body()).await?;
    let iso_data = String::from_utf8(body.to_vec()).unwrap();

    let iso_obj: Value = serde_json::from_str(&iso_data).unwrap();

    let r: DHIRequest = DHIRequest::new(iso_obj);
    let msg = r.serialize().unwrap();

    let res = talk_to_dhi_host(msg).await.unwrap(); // FIXME: unwrap 😱

    let mut res = Response::new(Body::from(res.serialize()));
    *res.status_mut() = StatusCode::OK;
    Ok(res)
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
        {
            Ok::<_, hyper::Error>(service_fn(serve_request))
        }
    }));

    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    run_server(addr).await;
}
*/

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::StreamExt;

async fn serve_dhi_request(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let x = body.next().await.unwrap()?;
    println!("{:?}", x);

    Ok(HttpResponse::Ok().finish())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/dhi", web::post().to(serve_dhi_request)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
