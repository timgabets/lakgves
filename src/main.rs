mod dhi;
use dhi::{DHIRequest, DHIResponse};

mod spxml;
use spxml::SPMessage;

mod errors;
use errors::AppError;

mod config;
use config::AppConfig;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use futures::StreamExt;
use serde_json::Value;
use serde_xml_rs::from_reader;
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "proust",
    about = "Application for testing Bank credit card processing systems."
)]
struct Opt {
    /// Configuration file
    #[structopt(parse(from_os_str))]
    config: PathBuf,
}

// TODO: Impl AppState
struct AppState {
    host_stream: TcpStream,
}

/// Asynchronously exchange data with DHI host
async fn talk_to_dhi_host(data: web::Data<AppState>, msg: String) -> Result<DHIResponse, AppError> {
    let mut s = &data.host_stream;
    let mut buffer = [0; 8192];

    // TODO: timeout from app state
    io::timeout(Duration::from_secs(5), async {
        s.write_all(&msg.as_bytes()).await?;
        s.read(&mut buffer).await?;
        Ok(())
    })
    .await?;

    // The first 5 bytes are the message length
    let response: DHIResponse = from_reader(&buffer[5..])?;
    Ok(response)
}

// TODO: write tests to cover all the unwrapping
async fn serve_dhi_request(
    data: web::Data<AppState>,
    mut body: web::Payload,
) -> Result<HttpResponse, Error> {
    let body = body.next().await.unwrap()?;
    let iso_data = String::from_utf8(body.to_vec()).unwrap();
    let iso_obj: Value = serde_json::from_str(&iso_data).unwrap();

    let r: DHIRequest = DHIRequest::new(iso_obj);
    let msg = r.serialize().unwrap();

    let res = talk_to_dhi_host(data, msg).await;
    match res {
        Ok(res) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .header("X-Hdr", "sample")
            .body(res.serialize().unwrap())),
        Err(err) => match err {
            AppError::IoError(err) => {
                println!("Error: {:?}", err);
                Ok(HttpResponse::GatewayTimeout()
                    .content_type("plain/text")
                    .body("Error communicating with DHI host"))
            }
            AppError::ParseError(err) => {
                println!("Error: {:?}", err);
                Ok(HttpResponse::InternalServerError()
                    .content_type("plain/text")
                    .body("Error processing data from DHI host"))
            }
            AppError::SerializeError(err) => {
                println!("Error: {:?}", err);
                Ok(HttpResponse::InternalServerError()
                    .content_type("plain/text")
                    .body("Serialization error"))
            }
            _ => {
                println!("Error: {:?}", err);
                Ok(HttpResponse::InternalServerError()
                    .content_type("plain/text")
                    .body("Internal error"))
            }
        },
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let cfg = AppConfig::new(opt.config.to_str().unwrap()).unwrap();

    // TODO: iterate through channels
    let dhi_host = &cfg.channels["dhi"]["host"].as_str().unwrap();
    let app_state = web::Data::new(AppState {
        host_stream: TcpStream::connect(dhi_host).await?,
    });

    app_state.host_stream.set_nodelay(true)?;
    println!("Connected to {}", dhi_host);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/dhi", web::post().to(serve_dhi_request))
    })
    .workers(cfg.get_num_of_workers())
    .keep_alive(cfg.get_listener_keep_alive())
    .bind(cfg.get_conn_str())?
    .run()
    .await
}
