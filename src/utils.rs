
pub fn get_localaddrs() -> Vec<String> {
    use local_ip_address::list_afinet_netifas;
    let network_interfaces = list_afinet_netifas().unwrap();
    let mut lo_infs = Vec::new();
    for inf in network_interfaces {
        let name = inf.0;
        let ip = inf.1;
        if ip.is_ipv4() {
            lo_infs.push(inf.1.to_string());
        }
    }
    return lo_infs;
}

pub fn get_available_port() -> u16 {
    use port_scanner::local_port_available;
    let lo_port = 51000;
    for lo_port in 51000..53000 {
        let is_available = local_port_available(lo_port);
        if is_available {
            break;
        }
    }
    return lo_port;
}