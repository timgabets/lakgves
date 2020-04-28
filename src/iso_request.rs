use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_xml_rs::{from_reader, to_string};

mod util;

#[derive(Serialize, Debug)]
pub struct Header {
    #[serde(rename(serialize = "MessageID"))]
    message_id: i64,
    #[serde(rename(serialize = "SystemID"))]
    system_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename(serialize = "RequestInput"))]
#[serde(rename(deserialize = "RequestResponse"))]
pub struct IsoRequest {
    #[serde(rename(serialize = "ISO8583-87"))]
    #[serde(rename(deserialize = "ISO8583-87"))]
    iso_fields: Value,
}

impl IsoRequest {
    pub fn new(iso_obj: Value) -> IsoRequest {
        let mut req = IsoRequest {
            iso_fields: iso_obj,
        };
        // TODO: check existing fields
        req.iso_fields["i007"] = serde_json::value::Value::String(util::get_mmddhhmmss());
        req.iso_fields["i011"] = serde_json::value::Value::String(util::gen_stan());
        req.iso_fields["i012"] = serde_json::value::Value::String(util::get_hhmmss());
        req.iso_fields["i013"] = serde_json::value::Value::String(util::get_mmdd());
        req.iso_fields["i037"] = serde_json::value::Value::String(util::gen_rrn());
        req
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
    #[ignore]
    fn dummy_request_serialization() {
        let iso_data = r#"{
            "i000": "0100",
            "i002": "521324******0895"
        }"#;

        let r: IsoRequest = IsoRequest::new(serde_json::from_str(&iso_data).unwrap());

        assert_eq!(r.serialize(), "00100<RequestInput><ISO8583-87><i000>0100</i000><i002>521324******0895</i002></ISO8583-87></RequestInput>");
    }

    #[test]
    fn dummy_deserialization() {
        let s = r##"
        <?xml version="1.0"?>
        <RequestResponse>
            <ISO8583-87><i000>0110</i000><i002>553691******0961</i002><i003>300000</i003><i004>000000000000</i004><i007>2804114717</i007><i043><![CDATA[IDDQD AM. 341215574     341215574 MSKRU]]></i043><i120>UD038IR0044444CR009ES0048100IA0103510198686</i120></ISO8583-87>"
        </RequestResponse>
        "##;

        let resp: IsoRequest = from_reader(s.as_bytes()).unwrap();
        assert_eq!(resp.iso_fields["i000"], json!({"$value": "0110" }));
        assert_eq!(
            resp.iso_fields["i002"],
            json!({"$value": "553691******0961" })
        );
        assert_eq!(resp.iso_fields["i003"], json!({"$value": "300000" }));
        assert_eq!(resp.iso_fields["i004"], json!({"$value": "000000000000" }));
        assert_eq!(resp.iso_fields["i007"], json!({"$value": "2804114717" }));
        assert_eq!(
            resp.iso_fields["i043"],
            json!({"$value": "IDDQD AM. 341215574     341215574 MSKRU" })
        );
        assert_eq!(
            resp.iso_fields["i120"],
            json!({"$value": "UD038IR0044444CR009ES0048100IA0103510198686" })
        );
    }
}
