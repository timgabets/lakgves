use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::Value;
use serde_xml_rs::to_string;

mod util;

#[derive(Serialize, Debug)]
pub struct Header {
    #[serde(rename(serialize = "MessageID"))]
    message_id: i64,
    #[serde(rename(serialize = "SystemID"))]
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

#[derive(Debug)]
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

impl Serialize for Request {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RequestInput", 2)?;
        state.serialize_field("FIXME", &self.header)?;
        state.serialize_field("ISO8583-87", &self.iso_fields)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_header_serialization() {
        let hdr = Header::new(1234, String::from("IDDQD"));
        assert_eq!(
            to_string(&hdr).unwrap(),
            "<Header><MessageID>1234</MessageID><SystemID>IDDQD</SystemID></Header>"
        );
    }

    #[test]
    fn dummy_request_serialization() {
        let iso_data = r#"{
            "i000": "0100",
            "i002": "521324******0895"
        }"#;

        let r: Request = Request::new(serde_json::from_str(&iso_data).unwrap());

        assert_eq!(r.serialize(), "<RequestInput><FIXME><Header><MessageID>430173293629234065</MessageID><SystemID>PROUST</SystemID></Header></FIXME><ISO8583-87><i000>0100</i000><i002>521324******0895</i002></ISO8583-87></RequestInput>");
    }
}
