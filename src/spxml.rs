use std::io::Write;
use yaserde::ser::to_string;
use yaserde::YaSerialize;

use crate::errors::AppError;

#[derive(YaSerialize, PartialEq, Debug)]
#[yaserde(rename = "IRIS")]
pub struct SPMessage {
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

impl SPMessage {
    pub fn serialize(&self) -> Result<String, AppError> {
        let serialized = to_string(self).unwrap();
        Ok(serialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_serialization() {
        let msg = SPMessage {
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
        /*
        <IRIS Version="1" Message="ModelRequest" MessageTypeId="60" MessageId="0af87c75503b4401">
        <msgSubType>aa</msgSubType>
        <msgType>aaaa</msgType>
        <msisdnA>231231</msisdnA>
        <msisdnB>54656456</msisdnB>
        <partNumber>127</partNumber>
        <partsQty>127</partsQty> <!-- Considering this field as unknown -->
        <sessionId>aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa</sessionId>
        <siebelId>aaaaaaaaaaaaaaaa</siebelId>
        <smsBody>aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa</smsBody>
        <smsId>aaa</smsId>
        <timestamp>2020-04-27 12:00:00</timestamp>
        <vlr>36028797018963968</vlr>
        </IRIS>
         */
        assert_eq!(
            msg.serialize().unwrap(),
            r#"<?xml version="1.0" encoding="utf-8"?><IRIS Version="1" Message="ModelRequest" MessageTypeId="60" MessageId="0af87c75503b4401"><msgSubType>iddqd</msgSubType><msgType>aaaa</msgType><msisdnA>231231</msisdnA><msisdnB>54656456</msisdnB><partNumber>127</partNumber><sessionId>bbbbb</sessionId><siebelId>ccccc</siebelId><smsBody>ddddd</smsBody><smsId>eee</smsId><timestamp>2020-04-27 12:00:00</timestamp><vlr>36028797018963968</vlr></IRIS>"#
        );
    }
}

/*

use crate::errors::AppError;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(deserialize = "IRIS"))]
#[serde(rename(serialize = "IRIS"))]
#[serde(rename_all = "camelCase")]
pub struct SPMessage {
    msg_sub_type: String,
    msg_type: String,
    msisdn_a: String,
    msisdn_b: String,
    part_number: String,
    session_id: String,
    siebel_id: String,
    sms_body: String,
    sms_id: String,
    timestamp: String,
    vlr: String,
}

impl SPMessage {
    pub fn serialize(&self) -> Result<String, AppError> {
        let serialized = serde_xml_rs::to_string(&self)?;
        Ok(serialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::from_reader;

    #[test]
    fn dummy_deserialization() {
        let s = r##"
        <IRIS Version="1" Message="ModelRequest" MessageTypeId="60" MessageId="0af87c75503b4401">
        <msgSubType>aa</msgSubType>
        <msgType>aaaa</msgType>
        <msisdnA>231231</msisdnA>
        <msisdnB>54656456</msisdnB>
        <partNumber>127</partNumber>
        <partsQty>127</partsQty> <!-- Considering this field as unknown -->
        <sessionId>aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa</sessionId>
        <siebelId>aaaaaaaaaaaaaaaa</siebelId>
        <smsBody>aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa</smsBody>
        <smsId>aaa</smsId>
        <timestamp>2020-04-27 12:00:00</timestamp>
        <vlr>36028797018963968</vlr>
        </IRIS>
        "##;

        let msg: SPMessage = from_reader(s.as_bytes()).unwrap();

        assert_eq!(msg.msg_sub_type, "aa");
        assert_eq!(msg.msg_type, "aaaa");
        assert_eq!(msg.msisdn_a, "231231");
        assert_eq!(msg.msisdn_b, "54656456");
        assert_eq!(msg.part_number, "127");
        assert_eq!(msg.session_id, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        assert_eq!(msg.siebel_id, "aaaaaaaaaaaaaaaa");
        assert_eq!(msg.sms_body, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        assert_eq!(msg.sms_id, "aaa");
        assert_eq!(msg.timestamp, "2020-04-27 12:00:00");
        assert_eq!(msg.vlr, "36028797018963968");
    }

    #[test]
    fn dummy_serialization() {
        let msg = SPMessage {
            msg_sub_type: String::from("iddqd"),
            msg_type: String::from("iddqd"),
            msisdn_a: String::from("iddqd"),
            msisdn_b: String::from("iddqd"),
            part_number: String::from("iddqd"),
            session_id: String::from("iddqd"),
            siebel_id: String::from("iddqd"),
            sms_body: String::from("iddqd"),
            sms_id: String::from("iddqd"),
            timestamp: String::from("iddqd"),
            vlr: String::from("iddqd"),
        };
        let serialized = msg.serialize().unwrap();
        assert_eq!(
            serialized,
            r#"<IRIS><msgSubType>iddqd</msgSubType><msgType>iddqd</msgType><msisdnA>iddqd</msisdnA><msisdnB>iddqd</msisdnB><partNumber>iddqd</partNumber><sessionId>iddqd</sessionId><siebelId>iddqd</siebelId><smsBody>iddqd</smsBody><smsId>iddqd</smsId><timestamp>iddqd</timestamp><vlr>iddqd</vlr></IRIS>"#
        );
    }
}
*/
