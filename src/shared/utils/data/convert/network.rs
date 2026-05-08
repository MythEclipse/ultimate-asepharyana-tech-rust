use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

/// Convert IPv4 string to u32.
pub fn ipv4_to_u32(ip: &str) -> Option<u32> {
    ip.parse::<Ipv4Addr>().ok().map(|addr| u32::from(addr))
}

/// Convert u32 to IPv4 string.
pub fn u32_to_ipv4(n: u32) -> String {
    Ipv4Addr::from(n).to_string()
}

/// Convert u32 to Ipv4Addr.
pub fn u32_to_ipv4_addr(n: u32) -> Ipv4Addr {
    Ipv4Addr::from(n)
}

/// Convert string to IpAddr.
pub fn str_to_ip_addr(s: &str) -> Option<IpAddr> {
    s.parse().ok()
}

/// Convert IpAddr to string.
pub fn ip_addr_to_string(ip: IpAddr) -> String {
    ip.to_string()
}

/// Convert string to SocketAddr.
pub fn str_to_socket_addr(s: &str) -> Option<SocketAddr> {
    s.parse().ok()
}

/// Convert SocketAddr to string.
pub fn socket_addr_to_string(addr: SocketAddr) -> String {
    addr.to_string()
}

/// Create SocketAddrV4 from IP and port.
pub fn ipv4_port_to_socket(ip: Ipv4Addr, port: u16) -> SocketAddrV4 {
    SocketAddrV4::new(ip, port)
}

/// Create SocketAddrV6 from IP and port.
pub fn ipv6_port_to_socket(ip: Ipv6Addr, port: u16) -> SocketAddrV6 {
    SocketAddrV6::new(ip, port, 0, 0)
}

/// Check if IP is loopback.
pub fn is_loopback(ip: &str) -> bool {
    ip.parse::<IpAddr>()
        .map(|a| a.is_loopback())
        .unwrap_or(false)
}

/// Check if IP is private (IPv4).
pub fn is_private_ip(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>()
        .map(|a| a.is_private())
        .unwrap_or(false)
}
