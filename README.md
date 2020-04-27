## Proust

HTTP to [ISO8583](https://en.wikipedia.org/wiki/ISO_8583) message converter. The main purpose of the system is testing Issuer Bank systems.

                           +-----------------+                    +--------------------+
                           |                 | <--  ISO 8583  --> |    Issuer Bank     |
    HTTP client <--JSON--> |     Proust      | <--VISA SMS/DMS--> |   Authorization    |
                           |                 | <-- Mastercard --> |        Host        |
                           +-----------------+                    +--------------------+

Приложение конвертирует данные, полученные из HTTP POST запроса, в сообщение в формате XML и посылает их на ISO host, e.g.:
> curl --header "Content-Type: application/json" --request POST --data '{"i000":"0100", "i002":"553691*****60961", "i003":"300000", "i004":"000000000000", "i014":"2402", "i018":"6011", "i022":"0100", "i023":"000", "i025":"02", "i026":"04", "i032":"437783", "i041":"TERMID01", "i042":"IDDQD MERCH ID", "i043":"IDDQD AM. 341215574     341215574 MSKRU", "i049":"643", "i053":"9801100001000000"}' localhost:8080

Please refer to your [ISO8583](https://en.wikipedia.org/wiki/ISO_8583) specification for the fields meaning and possible values.

Payload должен содержать json с необходимыми iso-полями. При отсутствии некоторых полей (например, DE 11, DE 37), они будут заполнены автоматически.