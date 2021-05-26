

fn main() {
    let listen_addr = String::from("127.0.0.1:8080");
    let port = &listen_addr[listen_addr.find(":").unwrap()+1..];

    dbg!(&listen_addr);
    dbg!(&port);
    // let http_server = HttpServer::new(.to_string());
    // http_server.run();
}

struct HttpServer {
    addr: String
}

impl HttpServer {
    fn new(addr: String) -> Self {
        Self{
            addr
        }
    }

    fn run(self) {
        
    }
}
