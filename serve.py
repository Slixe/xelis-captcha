import http.server
from http.server import SimpleHTTPRequestHandler
import socketserver

PORT = 8000
PUBLIC_FOLDER = "public"

class CustomHTTPRequestHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        # Set the directory to serve as the `public` folder
        super().__init__(*args, directory=PUBLIC_FOLDER, **kwargs)

    def end_headers(self):
        # Set the Cross-Origin-Opener-Policy header to 'same-origin'
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        # Set the Cross-Origin-Embedder-Policy header to 'require-corp' or 'credentialless'
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        super().end_headers()

with socketserver.TCPServer(("", PORT), CustomHTTPRequestHandler) as httpd:
    print(f"Serving on port {PORT}")
    httpd.serve_forever()