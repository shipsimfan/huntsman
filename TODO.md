# To Do
 1. Add cache option to "static-http"
 2. Add keep-alive and close support
 3. Add all standard MIME types to "static-http"
 4. Add compression to "static-http"
 5. Add support for content encodings on requests (Handle transparently)
 6. Add support for transfer encodings on requests
    Transparently handle compression. Pass through chunked. Add a collect command for chunked.
   1. chunked
   2. gzip
 7. Add support for absolute form of request target
 8. Add support for expectations/"100 Continue"
 9. Add support for WebSockets
     Add as additional request body type (aka. types will be: `Full`, `Chunked`, `WebSocket`)
 10. Implement different transports
   1. HTTPS
   2. HTTP2
   3. HTTP3