# save as server.py
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer

BLOCKED = {"secret.txt", "config.json", ".env", "server"}

class FilteredHandler(SimpleHTTPRequestHandler):
    def do_GET(self):
        # deny if the path ends with a blocked file
        if any(self.path.endswith(f) for f in BLOCKED):
            self.send_error(403, "Forbidden")
            return
        super().do_GET()

if __name__ == "__main__":
    ThreadingHTTPServer(("0.0.0.0", 80), FilteredHandler).serve_forever()