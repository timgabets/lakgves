## Proust

HTTP to [ISO8583](https://en.wikipedia.org/wiki/ISO_8583) message converter. The main purpose of the system is testing Issuer Bank systems.

                           +-----------------+                    +--------------------+
                           |                 | <--  ISO 8583  --> |    Issuer Bank     |
    HTTP client <--JSON--> |     Proust      | <--VISA SMS/DMS--> |   Authorization    |
                           |                 | <-- Mastercard --> |        Host        |
                           +-----------------+                    +--------------------+

Приложение конвертирует данные, полученные из HTTP POST запроса, в сообщение в формате XML и посылает их на ISO host, e.g.:
> curl --header "Content-Type: application/json" --request POST --data '{"i000":"0100","i002":"4444000011112222"}' localhost:8080

Payload должен содержать json с необходимыми iso-полями. При отсутствии некоторых полей (например, DE 11, DE 37), они будут заполнены автоматически.