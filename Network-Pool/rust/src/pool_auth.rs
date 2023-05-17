use ipnet::IpNet;
use cap_std::net::Pool;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use cap_std::ambient_authority;


pub fn create_tcp_port() -> Pool {
    let net: IpNet = "93.184.216.14/24".parse().unwrap();
    let mut tcp_pool = Pool::new();
    tcp_pool.insert_ip_net_port_range(net, 0, Some(65535), ambient_authority());
    tcp_pool
}

pub fn create_net_port() -> Pool {
    let mut pool = Pool::new();
    for x in 0..255 {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(93, 184, 216, x)), 80);
        pool.insert_socket_addr(socket, ambient_authority());
    }
    pool
}
