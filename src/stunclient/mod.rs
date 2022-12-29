use std::net::{IpAddr, SocketAddr, UdpSocket, TcpStream};
use bytecodec::{DecodeExt, EncodeExt};
use stun_codec::{Message, MessageClass, MessageDecoder, MessageEncoder, TransactionId, Attribute, rfc5389::attributes::MappedAddress};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

const STUN_SERVER_PORT: u16 = 3478;

pub struct Addresses {
    pub lo_source: String,
    pub mapped: String,
    pub xor_mapped: String,
}

pub fn udp_discovery(stunserver : IpAddr, port: u16) -> Result<Addresses, String> {
    // Create a UDP socket
    use port_scanner::local_port_available;
    let lo_port = 51000;
    for lo_port in 51000..53000 {
        let is_available = local_port_available(lo_port);
        if is_available {
            break;
        }
    }
    
    let lo_addr = format!("0.0.0.0:{}", lo_port);
    let socket = UdpSocket::bind(&lo_addr).unwrap();
    let server_addr = SocketAddr::new(stunserver, port);

    // Create a STUN message with the BINDING request type
    use stun_codec::rfc5389::{attributes::Software, methods::BINDING, Attribute};
    let mut message = Message::new(
        MessageClass::Request, 
        BINDING, 
        TransactionId::new([3; 12])
    );
    message.add_attribute(Attribute::Software(Software::new("foo".to_owned()).unwrap()));

    // Encode the STUN message into a byte array
    let mut encoder = MessageEncoder::new();
    let bytes = encoder.encode_into_bytes(message.clone()).unwrap();

    // Send the STUN request to the server
    socket.send_to(&bytes, server_addr).unwrap();

    // Wait for the STUN response
    let mut response_bytes = [0; 1024];
    let (bytes_received, _src_addr) = socket.recv_from(&mut response_bytes).unwrap();

    let mut addresses = Addresses {
        lo_source: String::from(lo_addr),
        mapped: "".to_string(),
        xor_mapped: "".to_string(),
    };
    // Decode the STUN response
    let mut decoder = MessageDecoder::<Attribute>::new();
    let to_decode = &response_bytes[..bytes_received];
    match decoder.decode_from_bytes(&to_decode) {
        Ok(attr_msg) => {
            let mut decoded = attr_msg.unwrap();
            for attr in decoded.attributes() {
                match attr {
                    Attribute::MappedAddress(ma) => {
                        addresses.mapped = ma.address().to_string();
                        // println!("MappedAddress: {}", addresses.mapped);
                    },
                    Attribute::XorMappedAddress(xma) => {
                        addresses.xor_mapped = xma.address().to_string();
                        // println!("XorMappedAddress: {}", addresses.xor_mapped);
                    },
                    _ => {}
                }
            }
        },
        Err(e) => println!("Err: {:?}", e),
    }
    return Ok(addresses);
}


pub async fn tcp_discovery(stunserver : IpAddr, port: u16) -> Result<Addresses, String> {
    use port_scanner::local_port_available;
    let lo_port = 51000;
    for lo_port in 51000..53000 {
        let is_available = local_port_available(lo_port);
        if is_available {
            break;
        }
    }
    
    let lo_addr = format!("0.0.0.0:{}", lo_port);
    let socket = tokio::net::TcpSocket::new_v4();
    socket.unwrap().bind(lo_addr.parse().unwrap()).unwrap();

    let server_addr = SocketAddr::new(stunserver, port);
    let mut stream = tokio::net::TcpStream::connect(server_addr).await.unwrap();

    // Create a STUN message with the BINDING request type
    use stun_codec::rfc5389::{attributes::Software, methods::BINDING, Attribute};
    let mut message = Message::new(
        MessageClass::Request, 
        BINDING, 
        TransactionId::new([3; 12])
    );
    message.add_attribute(Attribute::Software(Software::new("foo".to_owned()).unwrap()));

    // Encode the STUN message into a byte array
    let mut encoder = MessageEncoder::new();
    let bytes = encoder.encode_into_bytes(message.clone()).unwrap();

    // Send the STUN request to the server
    stream.write_all(&bytes).await.unwrap();

    // Wait for the STUN response
    let mut response_bytes = [0; 1024];
    let bytes_received = stream.read(&mut response_bytes).await.unwrap();

    let mut addresses = Addresses {
        lo_source: String::from(lo_addr),
        mapped: "".to_string(),
        xor_mapped: "".to_string(),
    };
    // Decode the STUN response
    let mut decoder = MessageDecoder::<Attribute>::new();
    let to_decode = &response_bytes[..bytes_received];
    match decoder.decode_from_bytes(&to_decode) {
        Ok(attr_msg) => {
            let mut decoded = attr_msg.unwrap();
            for attr in decoded.attributes() {
                match attr {
                    Attribute::MappedAddress(ma) => {
                        addresses.mapped = ma.address().to_string();
                    },
                    Attribute::XorMappedAddress(xma) => {
                        addresses.xor_mapped = xma.address().to_string();
                    },
                    _ => {}
                }
            }
        },
        Err(e) => println!("Err: {:?}", e),
    }
    return Ok(addresses);
}
