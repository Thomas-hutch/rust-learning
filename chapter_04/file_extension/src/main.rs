fn get_extension(filename: &str) -> &str {
    let bytes = filename.as_bytes();

    for (i, &item) in bytes.iter().enumerate().rev() {
        if item == b'.' {
            return &filename[i..];
        }
    }
    &filename
}

fn main() {
    let file = "image.png.txt";
    let ext = get_extension(file);

    println!("Extension: {}", ext);
}
