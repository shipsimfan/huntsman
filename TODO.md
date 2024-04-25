# To Do
 1. Add index support to "static-http"
 2. Add "static-http" command line arguments
 3. Add proper logging to "static-http"
 4. Add cache option to "static-http"
 5. Add keep-alive and close support
 6. Add compression to "static-http"
 7. Add support for content encodings on requests (Handle transparently)
 8. Add support for transfer encodings on requests
    Transparently handle compression. Pass through chunked. Add a collect command for chunked.
   1. chunked
   2. gzip
 9. Add support for chunked response bodies
     Use trait object with `async fn next(&mut self) -> Option<&[u8]>`
 10. Add support for absolute form of request target
 11. Add support for expectations/"100 Continue"
 12. Add support for WebSockets
     Add as additional request body type (aka. types will be: `Full`, `Chunked`, `WebSocket`)
 13. Implement different transports
   1. HTTPS
   2. HTTP2
   3. HTTP3
 14. Add all standard MIME types to "static-http"