use serde_json::{Value};
use serde_xml_rs::{to_string};
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Header {
	message_id : i64,
	system_id : String,
}

impl Header {
	pub fn new() -> Header {
		Header {message_id: Header::gen_message_id(), system_id: Header::gen_system_id()}
	}

	fn gen_message_id() -> i64 {
		// TODO: random generating
		430173293629234065
	}

	fn gen_system_id() -> String {
		// TODO: using from config file??
		String::from("PROUST")
	}
}

#[derive(Serialize, Debug)]
pub struct Request {
	header : Header,
	iso_fields : Value,
}

impl Request {
	pub fn new(iso_obj : Value) -> Request {
		Request {header: Header::new(), iso_fields: iso_obj}
	}

	pub fn serialize(&self) -> String {
		// TODO: return Result
		//to_string(&self.iso_fields).unwrap()
		to_string(&self).unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn request_serializetion_no_iso_tags() {
		let iso_data = r#"{}"#;

		let r : Request = Request::new(serde_json::from_str(&iso_data).unwrap());

		let serialized = r.serialize();
		assert_eq!(serialized, "");
	}
}
