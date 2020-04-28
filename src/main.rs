mod dhi;
use dhi::{DHIRequest, DHIResponse};

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use hyper::{Body, Client, Request, Response, Uri};
use serde_json::Value;
use std::net::SocketAddr;

use serde_xml_rs::from_reader;

use async_std::net::TcpStream;
use async_std::prelude::*;

async fn talk_to_iso_host(msg: String) -> Result<Response<Body>, std::io::Error> {
    let mut s = TcpStream::connect("10.217.13.27:10304").await?;
    s.write_all(&msg.as_bytes()).await?;
    println!("{}", msg);

    let mut buffer = [0; 2048]; // TODO: some more buffer size maybe?
    s.read(&mut buffer).await?;
    println!("{:?}", String::from_utf8_lossy(&buffer[5..]));

    /* Expected answer may look like this:
      "00782<?xml version=\"1.0\"?>\n<RequestResponse><Header/><Result><Code>0</Code><Description>OK</Description></Result><ISO8583-87><i000>0110</i000><i002>5536913798660961</i002><i003>300000</i003><i004>000000000000</i004><i007>2804114717</i007><i011>361637</i011><i012>114717</i012><i013>0428</i013><i014>2402</i014><i018>6011</i018><i022>0100</i022><i023>000</i023><i025>02</i025><i026>4</i026><i032>437783</i032><i037>710203141014</i037><i038>022578</i038><i039>00</i039><i041><![CDATA[TERMID01]]></i041><i042>IDDQD MERCH ID </i042><i043><![CDATA[IDDQD AM. 341215574     341215574 MSKRU]]></i043><i049>643</i049><i053>9801100001000000</i053><i054>0001643C099994300762</i054><i096>0000710203141014</i096><i120>UD038IR0044444CR009ES0048100IA0103510198686</i120></ISO8583-87></RequestResponse>\n"
    */

    let resp: DHIResponse = from_reader(&buffer[5..]).unwrap(); //TODO: unwrap 😱
    println!("{:?}", resp);

    // TODO: checking resp.res.code != 0

    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("Try POSTing data to /echo");

    // TODO: headers "content-type": "application/json", "content-length": "319"

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
    let msg = r.serialize();

    let res = talk_to_iso_host(msg).await;
    match res {
        Ok(res) => {
            println!("{:?}", res);
        }
        // TODO: return HTTP 502/503
        Err(err) => eprintln!("error: {}", err),
    }

    //
    let url = "http://www.rust-lang.org/en-US/"
        .parse::<Uri>()
        .expect("Error parsing URL");
    let res = Client::new().get(url).await?;

    // TODO: assigning HTTP status codes explicitly
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
