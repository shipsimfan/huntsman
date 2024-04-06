# To Do
 1. Fix erroneous "header is incomplete" error on disconnect
 2. Add TCP no-delay option for HTTP
 3. Add keep-alive checking (close after writing request if no keep-alive)
 4. Outline of different transports for HTTP, implementation not needed (HTTPS, HTTP2, HTTP3)
 5. Implement reading files for "static-http"
 6. "static-http" command line arguments
 7. "static-http" changeable error htmls
 8. Add support for transfer encodings on requests
   1. chunked
   2. gzip
 9. Add support for WebSockets
 10. Implement different transports
   1. HTTPS
   2. HTTP2
   3. HTTP3