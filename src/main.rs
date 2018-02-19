// TODO future -> handle things other than IPv4
// First iteration, make one successful ping request with hardcoded destination and payload

// TODO the IpHeader is big endian
#[derive(Debug)]
struct IpHeader {
    // first 32-bits
    version: u8, // TODO convert to 4 bits
    header_length: u8, // TODO convert to 4 bits
    type_of_service: u8, // 8 bits
    total_length: u16, // 16 bits
    // second 32-bits
    identification: u16, // 16 bits
    flags: u8, // need to end up being 3 bits
    fragment_offset: u8, // 13 bits
    // third 32-bits
    time_to_live: u8, // 8 bits
    protocol: u8, // 8 bits
    header_checksum: u16, // 16 bits
    // forth 32-bits
    source_address: Vec<u8>, // 32 bits
    // fifth 32-bits
    destination_address: Vec<u8>, // 32 bits
}

impl IpHeader {

    pub fn new() -> Self {
        IpHeader {
            version: 0x0800, // TODO correct value ??
            header_length: 84, // TODO this needs to be calculated properly
            type_of_service: 0x0,
            total_length: 98, // TODO this needs to be calculated properly
            identification: 1, // some number that increments for every datagram
            flags: 0b000, // TODO find the real value for this
            fragment_offset: 0, // TODO this needs to be dynamic, it starts at zero through
            time_to_live: 64, // TODO this should be configurable
            protocol: 1, // ICMP
            header_checksum: 6969, // TODO calculate this properly
            source_address: String::from("127.0.0.1").into_bytes(), // TODO this should be configurable (currently localhost)
            destination_address: String::from("216.58.200.110").into_bytes(), // TODO this should be configurable (currently google)
        }

    }
}


#[derive(Debug)]
struct IpDatagram {
    header: IpHeader,
    options: String, // 32 bit
    data: String, // what's the bits??
}

impl IpDatagram {

    pub fn new() -> Self {
        IpDatagram {
            header: IpHeader::new(),
            options: String::new(), // TODO as the name says, these are optional
            data: String::new(), // TODO this will be the ICMP header and payload
        }
    }

}

#[derive(Debug)]
struct PingPacket {
    ip_datagram: IpDatagram,
}

impl PingPacket {

    pub fn new() -> Self {
        // TODO
        // create an IP packet with the correct IP header
        PingPacket {
            ip_datagram: IpDatagram::new(),
        }
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        vec![] // TODO return the real bytes
    }
}

struct NetworkLayer;

impl NetworkLayer {

    pub fn new() -> Self {
        // TODO find the network interface we're going to use
        NetworkLayer {  }
    }

    pub fn send_packet(&self, packet: PingPacket) {
        println!("Sending packet: {:?}", packet.into_bytes());
        // TODO
    }

    pub fn listen_for_echo_response(&self) -> PingPacket {
        println!("Listening for echo responses");
        // TODO
        PingPacket::new()
    }
}

fn main() {
    let packet = PingPacket::new();
    let network_layer = NetworkLayer::new();
    network_layer.send_packet(packet);
    let echo_response = network_layer.listen_for_echo_response();
    println!("{:?}", echo_response);
}
