pub fn ip_to_u32(ip: &str) -> u32 {
    let parts: Vec<u8> = ip.split('.').map(|s| s.parse().unwrap()).collect();
    u32::from_be_bytes([parts[0], parts[1], parts[2], parts[3]])
}

pub fn u32_to_ip(ip: u32) -> String {
    let bytes = ip.to_be_bytes();
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}
