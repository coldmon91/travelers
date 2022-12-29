
use std::net::{IpAddr, ToSocketAddrs};

mod stunclient;

#[tokio::main]
async fn main() {
    // stunclient::discovery("stun.stunprotocol.org".to_string(), 3478);
    let domain_name = "stun.stunprotocol.org";
    println!("domain_name: {}", domain_name);
    let addr = (domain_name, 0).to_socket_addrs().unwrap().next().unwrap();


    let addrs = stunclient::tcp_discovery(addr.ip(), 3478).await.unwrap();
    println!("[tcp] \nlo_source: {} \nmapped: {} \nxor_mapped: {}", addrs.lo_source, addrs.mapped, addrs.xor_mapped);

    let addrs = stunclient::udp_discovery(addr.ip(), 3478).await.unwrap();
    println!("[udp] \nlo_source: {} \nmapped: {} \nxor_mapped: {}", addrs.lo_source, addrs.mapped, addrs.xor_mapped);

    //
}
