mod stunclient;
use tokio::runtime::Runtime;

#[no_mangle]
pub extern "C" fn tcp_discovery(stunserver : [u8; 64], port: i32) {
    match array_to_ipaddr(stunserver) {
        Ok(ip) => {
            let rt = Runtime::new().unwrap();
            rt.block_on(stunclient::tcp_discovery(ip, port as u16));
        },
        Err(e) => println!("Error: {}", e),
    }
}

#[no_mangle]
pub extern "C" fn udp_discovery(stunserver : [u8; 64], port: i32) {
    match array_to_ipaddr(stunserver) {
        Ok(ip) => {
            let rt = Runtime::new().unwrap();
            rt.block_on(stunclient::udp_discovery(ip, port as u16));
        },
        Err(e) => println!("Error: {}", e),
    }
}

fn array_to_ipaddr(ip_str: [u8; 64]) -> Result<std::net::IpAddr, String> {
    match std::str::from_utf8(&ip_str) {
        Ok(ip) => {
            match ip.parse() {
                Ok(ip) => Ok(ip),
                Err(_) => Err("Invalid IP address".to_string()),
            }
        },
        Err(_) => Err("Invalid IP address".to_string()),
    }
}
