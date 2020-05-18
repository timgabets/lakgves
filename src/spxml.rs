use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename(deserialize = "IRIS"))]
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
}
