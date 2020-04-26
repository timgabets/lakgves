//use serde_json::{Result, Value};

//use std::io::prelude::*;
//use std::net::TcpStream;

// mod request;
// use request::Request;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use hyper::{Body, Client, Request, Response, Uri};
use serde_json::Value;
use std::net::SocketAddr;

/*
pub fn handler(state: State) -> (State, Response<Body>) {
    let iso_data = r#"
        {
            "i000": "0100",
            "i002": "5536913******0961",
            "i003": "300000",
            "i004": "000000000000",
            "i014": "2402",
            "i018": "6011",
            "i022": "0100",
            "i023": "000",
            "i025": "02",
            "i026": "04",
            "i032": "437783",
            "i037": "293629234065",
            "i041": "TERMID01",
            "i042": "IDDQD MERCH ID",
            "i043": "IDDQD AM. 341215574     341215574 MSKRU",
            "i049": "643",
            "i053": "9801100001000000",
            "i120": "UD009TF0040431"
        }"#;

    let iso_obj: Value = serde_json::from_str(&iso_data).unwrap();

    let r: Request = Request::new(iso_obj);

    let msg = r.serialize();

    //let mut s = TcpStream::connect("10.217.13.27:10304").unwrap();
    //s.write(&msg.as_bytes()).expect("write() error");
    // println!("{}", msg);

    //let mut buffer = [0; 2048];
    //s.read(&mut buffer).expect("read() error");
    //println!("recv: {}", String::from_utf8_lossy(&buffer[..]));

    let res = create_response(&state, StatusCode::OK, mime::TEXT_PLAIN, msg);
    (state, res)
}

fn router() -> Router {
    build_simple_router(|route| {
        route.post("/").to(handler);
    })
}

pub fn main() {
    let addr = "127.0.0.1:8080";
    println!("Listening to {}", addr);
    gotham::start(addr, router())
}
*/

async fn serve_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Got request at {:?}", req.uri());
    println!("Headers: {:?}", req.headers());

    let body = hyper::body::to_bytes(req.into_body()).await?;
    let iso_data = String::from_utf8(body.to_vec()).unwrap();

    let iso_obj: Value = serde_json::from_str(&iso_data).unwrap();
    println!("{:?}", iso_obj);

    let url = "http://www.rust-lang.org/en-US/"
        .parse::<Uri>()
        .expect("Error parsing URL");
    let res = Client::new().get(url).await?;
    println!("request finished -- returning response");
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    run_server(addr).await;
}
