use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::Value;
use serde_xml_rs::to_string;

mod util;

#[derive(Debug)]
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

impl Serialize for Header {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Header", 2)?;
        state.serialize_field("MessageID", &self.message_id)?;
        state.serialize_field("SystemID", &self.system_id)?;
        state.end()
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
}
