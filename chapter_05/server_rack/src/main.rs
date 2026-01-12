fn main() {
    let mut rack = Rack::new(2);

    let s1 = Server::new("192.168.1.1");
    let s2 = Server::new("192.168.1.2");
    let s3 = Server::new("192.168.1.3"); // Limit exceeded

    println!("Added s1: {}", rack.add_server(s1));
    println!("Added s2: {}", rack.add_server(s2));
    println!("Added s3: {}", rack.add_server(s3)); // Should be false

    rack.list_active();

    rack.decommission();
}

struct Server {
    ip: String,
    is_online: bool,
}

struct Rack {
    servers: Vec<Server>,
    capacity: usize,
}

impl Server {
    fn new(ip: &str) -> Self {
        Server {
            ip: ip.to_string(),
            is_online: true,
        }
    }
}

impl Rack {
    fn new(capacity: usize) -> Self {
        Rack {
            servers: vec![],
            capacity,
        }
    }
    fn add_server(&mut self, server: Server) -> bool {
        if self.servers.len() < self.capacity {
            self.servers.push(server);
            true
        } else {
            println!("Rack at capacity!");
            false
        }
    }
    fn list_active(&self) {
        for server in &self.servers {
            if server.is_online {
                println!("IP: {}", server.ip);
            }
        }
    }
    fn decommission(self) {
        println!("decomissioning Rack");
        for server in self.servers {
            println!("Wiping [{}]", server.ip);
        }
    }
}
