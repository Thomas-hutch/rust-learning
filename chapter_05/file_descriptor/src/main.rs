fn main() {
    let f = File::new("log.txt");

    f.read();
    f.close();
}

struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> Self {
        File {
            name: name.to_string(),
            data: Vec::new(),
        }
    }
    fn read(&self) {
        println!("Reading {}", self.name)
    }
    fn close(self) {
        println!("Closing {}", self.name)
    }
}
