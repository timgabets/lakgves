extern crate gotham;
extern crate mime;
extern crate serde;
extern crate serde_derive;
extern crate serde_xml_rs;

//use serde_json::{Result, Value};
use serde_json::Value;

//use std::io::prelude::*;
//use std::net::TcpStream;

mod request;
use request::Request;

use gotham::helpers::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;

use hyper::{Body, Response, StatusCode};

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
