# To Do
 1. Fix erroneous "header is incomplete" error on disconnect
 2. Add keep-alive checking (close after writing request if no keep-alive)
 3. Outline of different transports for HTTP, implementation not needed (HTTPS, HTTP2, HTTP3)
 4. Implement reading files for "static-http"
 5. "static-http" command line arguments
 6. "static-http" changeable error htmls
 7. Add support for transfer encodings on requests
   1. chunked
   2. gzip
 8. Add support for WebSockets
 9. Implement different transports
   1. HTTPS
   2. HTTP2
   3. HTTP3