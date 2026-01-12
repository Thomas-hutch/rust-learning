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
            data: vec![4],
        }
    }
    fn read(&self) {
        println!("Reading {}: {}", self.name, self.data[0])
    }
    fn close(self) {
        println!("Closing {}", self.name)
    }
}
