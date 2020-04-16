use serde_json::{Value};
use serde_xml_rs::{to_string};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Request {
	id : i32,
	header : Header,
	iso_fields : Value,
}

impl Request {
	pub fn new(iso_obj : Value) -> Request {
		Request {id: 300, header: Header::new(), iso_fields: iso_obj}
	}

	pub fn serialize(&self) -> String {
		// TODO: return Result
		to_string(&self.iso_fields).unwrap()
	}
}

/*
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn dummy_gen_message_id() {
		assert_eq!(gen_message_id(), 430173293629234065);
	}

	#[test]
	fn dummy_gen_system_id() {
		assert_eq!(gen_system_id(), "PROUST");
	}
}
*/
