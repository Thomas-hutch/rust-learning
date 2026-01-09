fn main() {
    let packet = String::from("PAYLOAD:");
    let payload = extract_payload(&packet);
    println!("{payload}");
}

fn extract_payload(packet: &str) -> &str {
    let key = "PAYLOAD:";

    match packet.find(key) {
        Some(index) => {
            let data_start = index + key.len();
            let remaining = &packet[data_start..];
            match remaining.find('|') {
                Some(pipe_index) => &remaining[..pipe_index],
                None => remaining,
            }
        }
        None => "",
    }
}
