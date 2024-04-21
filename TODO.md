# To Do
 1. Combine content-type and body into a struct
 2. Change http address from `SocketAddr` to an enum for different protocols 
 3. Add index support to "static-http"
 4. Add "static-http" command line arguments
 5. Add proper logging to "static-http"
 6. Add cache option to "static-http"
 7. Add keep-alive and close support
 8. Add compression to "static-http"
 9. Add support for content encodings on requests (Handle transparently)
 10. Add support for transfer encodings on requests
    Transparently handle compression. Pass through chunked. Add a collect command for chunked.
   1. chunked
   2. gzip
 11. Add support for chunked response bodies
     Use trait object with `async fn next(&mut self) -> Option<&[u8]>`
 12. Add support for absolute form of request target
 13. Add support for expectations/"100 Continue"
 14. Add support for WebSockets
     Add as additional request body type (aka. types will be: `Full`, `Chunked`, `WebSocket`)
 15. Implement different transports
   1. HTTPS
   2. HTTP2
   3. HTTP3
 16. Add all standard MIME types to "static-http"