use serde::Serialize;
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

#[derive(Serialize, Debug)]
#[serde(rename(serialize = "RequestInput"))]
pub struct Request {
    #[serde(rename(serialize = "ISO8583-87"))]
    iso_fields: Value,
}

impl Request {
    pub fn new(iso_obj: Value) -> Request {
        Request {
            iso_fields: iso_obj,
        }
    }

    pub fn serialize(&self) -> String {
        // TODO: return Result
        let serialized = to_string(&self).unwrap();
        let serialized = format!("{:05}{}", serialized.len(), serialized);
        serialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_request_serialization() {
        let iso_data = r#"{
            "i000": "0100",
            "i002": "521324******0895"
        }"#;

        let r: Request = Request::new(serde_json::from_str(&iso_data).unwrap());

        assert_eq!(r.serialize(), "00100<RequestInput><ISO8583-87><i000>0100</i000><i002>521324******0895</i002></ISO8583-87></RequestInput>");
    }
}
