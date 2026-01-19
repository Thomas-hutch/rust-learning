use refactor_assignment::*;

fn main() {
    let input = r#"
        name = "MyApp"
        port = 8080
        debug = true
        # This is a comment
        timeout = 30
    "#;
    
    match load_config(input) {
        Ok(config) => {
            println!("Name: {:?}", config.get("name"));
            println!("Port: {:?}", config.get("port"));
        }
        Err(e) => println!("Error: {}", e),
    }
}