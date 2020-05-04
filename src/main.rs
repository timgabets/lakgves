mod dhi;
use dhi::{DHIRequest, DHIResponse};

mod errors;
use errors::AppError;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use async_std::net::TcpStream;
use async_std::prelude::*;
use futures::StreamExt;
use serde_json::Value;
use serde_xml_rs::from_reader;
use std::sync::Mutex;

struct AppState {
    counter: Mutex<i32>,
}

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

async fn serve_dhi_request(
    data: web::Data<AppState>,
    mut body: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    println!("Number of requests: {}", counter);

    let body = body.next().await.unwrap()?;
    let iso_data = String::from_utf8(body.to_vec()).unwrap();
    let iso_obj: Value = serde_json::from_str(&iso_data).unwrap();

    let r: DHIRequest = DHIRequest::new(iso_obj);
    let msg = r.serialize().unwrap();

    let res = talk_to_dhi_host(msg).await;
    match res {
        Ok(res) => {
            println!("OK");
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .header("X-Hdr", "sample")
                .body(res.serialize()))
        }
        Err(err) => match err {
            AppError::IoError(err) => {
                println!("Error: {:?}", err);
                Ok(HttpResponse::ServiceUnavailable()
                    .content_type("plain/text")
                    .body("Error communicating with DHI host"))
            }
            AppError::ParseError(err) => {
                println!("Error: {:?}", err);
                Ok(HttpResponse::InternalServerError()
                    .content_type("plain/text")
                    .body("Error processing data from DHI host"))
            }
        },
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/dhi", web::post().to(serve_dhi_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
