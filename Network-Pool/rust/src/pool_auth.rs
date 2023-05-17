use ipnet::IpNet;
use cap_std::net::Pool;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use cap_std::ambient_authority;

pub fn create_net_port() -> Pool {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(93, 184, 216, 34)), 80);
    let mut default_pool = Pool::new();
    default_pool.insert_socket_addr(socket, ambient_authority());
    default_pool
}

pub fn create_tcp_port() -> Pool {
    let net: IpNet = "93.184.216.34/24".parse().unwrap();
    let mut net_pool = Pool::new();
    net_pool.insert_ip_net_port_range(net, 0, Some(65535), ambient_authority());
    net_pool
}
