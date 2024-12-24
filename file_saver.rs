use std::fs::OpenOptions;
use std::io::Write;

pub fn save_client_data(client_id: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("clients.txt")
        .unwrap();

    writeln!(file, "Client ID: {}", client_id).unwrap();
}
