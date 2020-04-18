## Proust

HTTP to [ISO8583](https://en.wikipedia.org/wiki/ISO_8583) message converter. The main purpose of the system is testing Issuer Bank systems.

                           +-----------------+                    +--------------------+
                           |                 | <--  ISO 8583  --> |    Issuer Bank     |
    HTTP client <--JSON--> |     Proust      | <--VISA SMS/DMS--> |   Authorization    |
                           |                 | <-- Mastercard --> |        Host        |
                           +-----------------+                    +--------------------+

## Testing with curl
> curl --header "Content-Type: application/json" --request GET --data '{"i000":"0100","i002":"4444000011112222"}' localhost:8080
