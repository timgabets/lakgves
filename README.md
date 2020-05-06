## LACKGVES
[![Build Status](https://travis-ci.org/timgabets/lakgves.svg?branch=master)](https://travis-ci.org/timgabets/lakgves)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

### Concept

HTTP to [ISO8583](https://en.wikipedia.org/wiki/ISO_8583) message converter. The main purpose of the system is testing Issuer Bank systems.

                               +-----------------+                        +--------------------+
                               |                 | <---   ISO 8583   ---> |    Issuer Bank     |
    HTTP client <--- JSON ---> |    Lackgves     | <--- VISA SMS/DMS ---> |   Authorization    |
                               |                 | <---  Mastercard  ---> |        Host        |
                               +-----------------+                        +--------------------+

The application converts JSON payload to ISO8583 message in a proper format, sends it to the Bank host, receives the response and sends it back to the HTTP client.
In other words, the input JSON like this (note that the card number in i002 and expiration date in i014 are masked here for security reasons. The real request should contain the full unmacked data):
```json
{
	"i000":"0100",
	"i002":"443322******0961",
	"i003":"300000",
	"i004":"000000000000",
	"i014":"****",
	"i018":"6011",
	"i022":"0100",
	"i023":"000",
	"i025":"02",
	"i026":"04",
	"i032":"999999",
	"i041":"TERMID01",
	"i042":"IDDQD MERCH ID",
	"i043":"IDDQD AM. 341215574     341215574 MSKRU",
	"i049":"643",
	"i053":"9801100001000000"
}
```
will be converted into one of the [ISO8583](https://en.wikipedia.org/wiki/ISO_8583) flavours (e.g. Debit Host Interface XML in the example below):
```xml
<RequestInput>
	<ISO8583-87>
		<i000>0100</i000>
		<i002>443322******0961</i002>
		<i003>300000</i003>
		<i004>000000000000</i004>
		<i007>0505151908</i007> <!-- Transmission date & time - generated by the application if not provided -->
		<i011>367298</i011> <!-- STAN - generated by the application if not provided -->
		<i012>151908</i012> <!-- hhmmss - generated by the application if not provided -->
		<i013>0505</i013> <!-- MMDD - generated by the application if not provided -->
		<i014>****</i014>
		<i018>6011</i018>
		<i022>0100</i022>
		<i023>000</i023>
		<i025>02</i025>
		<i026>04</i026>
		<i032>999999</i032>
		<i037>964800787298</i037> <!-- RRN (Retrieval Reference Number) - generated by the application if not provided -->
		<i041>TERMID01</i041>
		<i042>IDDQD MERCH ID</i042>
		<i043>IDDQD AM. 341215574     341215574 MSKRU</i043>
		<i049>643</i049>
		<i053>9801100001000000</i053>
	</ISO8583-87>
</RequestInput>
```


Please refer to your [ISO8583](https://en.wikipedia.org/wiki/ISO_8583) specification for the fields meaning and possible values.

### Usage
Any HTTP tool (e.g. [curl](https://curl.haxx.se/) or [SoapUI](https://www.soapui.org/)) may be used to interact with the Lackgves app: 
> curl -v --header "Content-Type: application/json" --request POST --data '{"i000":"0100", "i002":"443322******0961", "i003":"300000", "i004":"000000000000", "i014":"****", "i018":"6011", "i022":"0100", "i023":"000", "i025":"02", "i026":"04", "i032":"999999", "i041":"TERMID01", "i042":"IDDQD MERCH ID", "i043":"IDDQD AM. 341215574     341215574 MSKRU", "i049":"643", "i053":"9801100001000000"}' localhost:8080

The received response may look like this:
```
< HTTP/1.1 200 OK
< content-length: 528
< content-type: application/json
< x-hdr: sample
< date: Tue, 05 May 2020 15:01:52 GMT
<
* Connection #0 to host localhost left intact
{"i000":"0110","i002":"443322******0961","i003":"300000","i004":"000000000000","i007":"0505180152","i011":"233005","i012":"180152","i013":"0505","i014":"****","i018":"6011","i022":"0100","i023":"000","i025":"02","i026":"4","i032":"999999","i037":"634538143910","i038":"082673","i039":"00","i041":"TERMID01","i042":"IDDQD MERCH ID","i043":"IDDQD AM. 341215574     341215574 MSKRU","i049":"643","i053":"9801100001000000","i054":"0001643D000001431065","i096":"0000634538143910","i120":"UD038IR0044444CR009ES0048100IA0103510301943"}* Closing connection 0
curl -v --header "Content-Type: application/json" --request POST --data
```