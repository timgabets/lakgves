use std::io::{Read, Write};
use yaserde::de::from_reader;
use yaserde::ser::to_string;
use yaserde::{YaDeserialize, YaSerialize};

use crate::errors::AppError;
use rand::Rng;

#[derive(YaSerialize, YaDeserialize, PartialEq, Debug)]
#[yaserde(rename = "IRIS")]
pub struct SPRequest {
    #[yaserde(attribute)]
    #[yaserde(rename = "Version")]
    version: u8,
    #[yaserde(attribute)]
    #[yaserde(rename = "Message")]
    message: String,
    #[yaserde(attribute)]
    #[yaserde(rename = "MessageTypeId")]
    message_type_id: u8,
    #[yaserde(attribute)]
    #[yaserde(rename = "MessageId")]
    message_id: String,

    #[yaserde(rename = "msgSubType")]
    msg_sub_type: String,

    #[yaserde(rename = "msgType")]
    msg_type: String,

    #[yaserde(rename = "msisdnA")]
    msisdn_a: u64,

    #[yaserde(rename = "msisdnB")]
    msisdn_b: u64,

    #[yaserde(rename = "partNumber")]
    part_number: u32,

    #[yaserde(rename = "sessionId")]
    session_id: String,

    #[yaserde(rename = "siebelId")]
    siebel_id: String,

    #[yaserde(rename = "smsBody")]
    sms_body: String,

    #[yaserde(rename = "smsId")]
    sms_id: String,

    #[yaserde(rename = "timestamp")]
    timestamp: String,

    #[yaserde(rename = "vlr")]
    vlr: String,
}

impl SPRequest {
    pub fn new(s: &[u8]) -> Self {
        let req: SPRequest = from_reader(s).unwrap();
        req
    }

    pub fn serialize(&self) -> Result<String, AppError> {
        let s = to_string(self).unwrap();
        // Removing leading <?xml version="1.0" encoding="utf-8"?>
        // TODO: more sophisticated removal solution using xml-rs
        Ok(s.replace("<?xml version=\"1.0\" encoding=\"utf-8\"?>", ""))
    }

    pub fn gen_message_id(&mut self) {
        let mut rng = rand::thread_rng();
        let msg_id: u64 = rng.gen();
        let msg_id = format!("{:08x}", msg_id);
        msg_id.to_string();
        self.message_id = msg_id.to_string();
    }
}

#[derive(YaSerialize, YaDeserialize, PartialEq, Debug)]
#[yaserde(rename = "IRIS")]
pub struct SPResponse {
    #[yaserde(attribute)]
    #[yaserde(rename = "Version")]
    version: u8,
    #[yaserde(attribute)]
    #[yaserde(rename = "Message")]
    message: String,
    #[yaserde(attribute)]
    #[yaserde(rename = "MessageTypeId")]
    message_type_id: u8,
    #[yaserde(attribute)]
    #[yaserde(rename = "MessageId")]
    message_id: String,
    #[yaserde(attribute)]
    #[yaserde(rename = "IrisInstance")]
    iris_instance: String,
    #[yaserde(attribute)]
    #[yaserde(rename = "SystemTime")]
    system_time: String,
    #[yaserde(attribute)]
    #[yaserde(rename = "UniqueRecordId")]
    uniq_record_id: u32,
    #[yaserde(attribute)]
    #[yaserde(rename = "Merging")]
    merging: u8,
    #[yaserde(attribute)]
    #[yaserde(rename = "InstanceStatus")]
    instance_status: String,
    #[yaserde(attribute)]
    #[yaserde(rename = "Latency")]
    latency: f32,
    #[yaserde(attribute)]
    #[yaserde(rename = "ErrorCode")]
    err_code: i32,
}

impl SPResponse {
    pub fn new(s: &[u8]) -> Self {
        let resp: SPResponse = from_reader(s).unwrap();
        resp
    }

    pub fn serialize(&self) -> Result<String, AppError> {
        let s = to_string(self).unwrap();
        // Removing leading <?xml version="1.0" encoding="utf-8"?>
        // TODO: more sophisticated removal solution using xml-rs
        Ok(s.replace("<?xml version=\"1.0\" encoding=\"utf-8\"?>", ""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_serialization() {
        let msg = SPRequest {
            version: 1,
            message: String::from("ModelRequest"),
            message_type_id: 60,
            message_id: String::from("0af87c75503b4401"),
            msg_sub_type: String::from("iddqd"),
            msg_type: String::from("aaaa"),
            msisdn_a: 231231,
            msisdn_b: 54656456,
            part_number: 127,
            session_id: String::from("bbbbb"),
            siebel_id: String::from("ccccc"),
            sms_body: String::from("ddddd"),
            sms_id: String::from("eee"),
            timestamp: String::from("2020-04-27 12:00:00"),
            vlr: String::from("36028797018963968"),
        };

        assert_eq!(
            msg.serialize().unwrap(),
            r#"<IRIS Version="1" Message="ModelRequest" MessageTypeId="60" MessageId="0af87c75503b4401"><msgSubType>iddqd</msgSubType><msgType>aaaa</msgType><msisdnA>231231</msisdnA><msisdnB>54656456</msisdnB><partNumber>127</partNumber><sessionId>bbbbb</sessionId><siebelId>ccccc</siebelId><smsBody>ddddd</smsBody><smsId>eee</smsId><timestamp>2020-04-27 12:00:00</timestamp><vlr>36028797018963968</vlr></IRIS>"#
        );
    }

    #[test]
    fn dummy_request_deserialization() {
        let s = r#"
        <IRIS Version="1" Message="ModelRequest" MessageTypeId="60" MessageId="0af87c75503b4401">
            <msgSubType>iddqd</msgSubType>
            <msgType>aaaa</msgType>
            <msisdnA>231231</msisdnA>
            <msisdnB>54656456</msisdnB>
            <partNumber>127</partNumber>
            <sessionId>bbbbb</sessionId>
            <siebelId>ccccc</siebelId>
            <smsBody>ddddd</smsBody>
            <smsId>eee</smsId>
            <timestamp>2020-04-27 12:00:00</timestamp>
            <vlr>36028797018963968</vlr>
        </IRIS>"#;

        let mut req: SPRequest = from_reader(s.as_bytes()).unwrap();

        assert_eq!(req.version, 1);
        assert_eq!(req.message, "ModelRequest");
        assert_eq!(req.message_type_id, 60);
        assert_eq!(req.message_id, "0af87c75503b4401");
        assert_eq!(req.msg_sub_type, "iddqd");
        assert_eq!(req.msg_type, "aaaa");
        assert_eq!(req.msisdn_a, 231231);
        assert_eq!(req.msisdn_b, 54656456);
        assert_eq!(req.part_number, 127);
        assert_eq!(req.session_id, "bbbbb");
        assert_eq!(req.siebel_id, "ccccc");
        assert_eq!(req.sms_body, "ddddd");
        assert_eq!(req.sms_id, "eee");
        assert_eq!(req.timestamp, "2020-04-27 12:00:00");
        assert_eq!(req.vlr, "36028797018963968");

        req.gen_message_id();
        assert_ne!(
            req.message_id, "0af87c75503b4401",
            "Message ID should have been changed"
        );
    }

    #[test]
    fn sp_request_new() {
        let s = r#"
        <IRIS Version="1" Message="ModelRequest" MessageTypeId="60" MessageId="0af87c75503b4401">
            <msgSubType>iddqd</msgSubType>
            <msgType>aaaa</msgType>
            <msisdnA>231231</msisdnA>
            <msisdnB>54656456</msisdnB>
            <partNumber>127</partNumber>
            <sessionId>bbbbb</sessionId>
            <siebelId>ccccc</siebelId>
            <smsBody>ddddd</smsBody>
            <smsId>eee</smsId>
            <timestamp>2020-04-27 12:00:00</timestamp>
            <vlr>36028797018963968</vlr>
        </IRIS>"#;

        let req = SPRequest::new(s.as_bytes());
        assert_eq!(req.version, 1);
    }

    #[test]
    fn dummy_responce_deserialization() {
        let s = r##"
        <IRIS Version="1" Message="ModelResponse" IrisInstance="INSTANCE_1_(DS-PR-" MessageTypeId="60" SystemTime="2020-05-18 23:39:19" UniqueRecordId="1882261" MessageId="0af87c75503b4401" Merging="0" InstanceStatus="Ok" Latency="1.15" ErrorCode="0"></IRIS>
        "##;

        let resp = SPResponse::new(s.as_bytes());
        assert_eq!(resp.version, 1);
        assert_eq!(resp.message, "ModelResponse");
        assert_eq!(resp.iris_instance, "INSTANCE_1_(DS-PR-");
        assert_eq!(resp.message_type_id, 60);
        assert_eq!(resp.system_time, "2020-05-18 23:39:19");
        assert_eq!(resp.uniq_record_id, 1882261);
        assert_eq!(resp.message_id, "0af87c75503b4401");
        assert_eq!(resp.merging, 0);
        assert_eq!(resp.instance_status, "Ok");
        assert_eq!(resp.latency, 1.15);
        assert_eq!(resp.err_code, 0);
    }
}
