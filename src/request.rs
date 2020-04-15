use serde::Serialize;
use serde_json::Value;
use serde_xml_rs::to_string;

mod util;

#[derive(Serialize, Debug)]
pub struct Header {
    message_id: i64,
    system_id: String,
}

impl Header {
    pub fn new(message_id: i64, system_id: String) -> Header {
        Header {
            message_id: message_id,
            system_id: system_id,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Request {
    header: Header,
    iso_fields: Value,
}

impl Request {
    pub fn new(iso_obj: Value) -> Request {
        Request {
            header: Header::new(util::gen_message_id(), util::get_system_id()),
            iso_fields: iso_obj,
        }
    }

    pub fn serialize(&self) -> String {
        // TODO: return Result
        //to_string(&self.iso_fields).unwrap()
        to_string(&self).unwrap()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_serialization_no_iso_tags() {
        let iso_data = r#"{}"#;

        let r : Request = Request::new(serde_json::from_str(&iso_data).unwrap());

        let serialized = r.serialize();
        assert_eq!(serialized, "");
    }
}
*/
