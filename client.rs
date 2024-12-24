pub fn validate_client(client_id: &str) -> bool {
    match client_id.parse::<u64>() {
        Ok(id) if id >= 1000 && id <= 9999 => true,
        _ => false,
    }
}
