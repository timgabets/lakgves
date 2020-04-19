extern crate serde;
extern crate serde_derive;
extern crate serde_xml_rs;

use serde_json::{Result, Value};

use std::io::prelude::*;
use std::net::TcpStream;

mod request;
use request::Request;

fn main() -> Result<()> {
    // Some day it is gonna be data parsed from the incoming http-request
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

    let iso_obj: Value = serde_json::from_str(&iso_data)?;

    let r: Request = Request::new(iso_obj);

    let msg = r.serialize();

    let mut s = TcpStream::connect("10.217.13.27:10304").unwrap();
    s.write(&msg.as_bytes()).expect("write() error");
    println!("{}", msg);

    let mut buffer = [0; 2048];
    s.read(&mut buffer).expect("read() error");
    println!("recv: {}", String::from_utf8_lossy(&buffer[..]));

    Ok(())
}
