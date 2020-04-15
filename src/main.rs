extern crate serde;
extern crate serde_derive;
extern crate serde_xml_rs;

use serde_json::{Result, Value};

mod request;
use request::Request;

fn main() -> Result<()> {
    // Some day it is gonna be data parsed from the incoming http-request
    let iso_data = r#"
		{
			"i000": "0100",
			"i002": "521324******0895",
			"i003": "000000",
			"i004": "000000001337",
			"i007": "2101165825",
			"i011": "430173",
			"i012": "165825",
			"i013": "0121",
			"i014": "2402",
			"i018": "6011",
			"i022": "0100",
			"i023": "000",
			"i025": "02",
			"i026": "04",
			"i032": "437783",
			"i037": "293629234065",
			"i041": "TERMID01",
			"i042": "IDDQD MERCH ID",
			"i043": "IDDQD AM. 341215574     341215574 MSKRU",
			"i049": "643",
			"i053": "9801100001000000",
			"i120": "UD009TF0040431"
		}"#;

    let iso_obj: Value = serde_json::from_str(&iso_data)?;

    let r: Request = Request::new(iso_obj);

    let serialized = r.serialize();
    println!("{}", serialized);

    Ok(())
}

/*
<RequestInput>
    <Header>
        <MessageID>430173293629234065</MessageID>
        <SystemID>IDDQD</SystemID>
    </Header>
    <ISO8583-87>
        <i000>0100</i000>
        <i002>521324******0895</i002>
        <i003>300000</i003>
        <i004>000000000000</i004>
        <i007>2101165825</i007>
        <i011>430173</i011>
        <i012>165825</i012>
        <i013>0121</i013>
        <i014>2402</i014>
        <i018>6011</i018>
        <i022>0100</i022>
        <i023>000</i023>
        <i025>02</i025>
        <i026>04</i026>
        <i032>437783</i032>
        <i037>293629234065</i037>
        <i041>TERMID01</i041>
        <i042>IDDQD MERCH ID</i042>
        <i043>IDDQD AM. 341215574     341215574 MSKRU</i043>
        <i049>643</i049>
        <i053>9801100001000000</i053>
        <i120>UD009TF0040431</i120>
    </ISO8583-87>
</RequestInput>
*/
