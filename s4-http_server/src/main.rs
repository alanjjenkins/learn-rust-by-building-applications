

fn main() {
    let http_server = HttpServer::new("127.0.0.1:8080");
    http_server.run();
}

struct HttpServer {
    addr: String
}

impl HttpServer {
    fn new(address: String) {
    }
}
