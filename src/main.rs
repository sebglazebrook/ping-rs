// TODO future -> handle things other than IPv4
// First iteration, make one successful ping request with hardcoded destination and payload

// TODO the IpHeader is big endian
#[derive(Debug)]
struct IpHeader {
    // first 32-bits
    version: String, // 4bits
    header_length: String, // 4bits
    type_of_service: String, // 8 bits
    total_length: String, //16 bits
    // second 32-bits
    identification: String, // 16 bits
    flags: String, // 3 bits
    fragment_offset: String, // 13 bits
    // third 32-bits
    time_to_live: String, // 8 bits
    protocol: String, // 8 bits
    header_checksum: String, // 16 bits
    // forth 32-bits
    source_address: String, // 32 bits
    // fifth 32-bits
    destination_address: String, // 32 bits
}

impl IpHeader {

    pub fn new() -> Self {
        IpHeader {
            version: String::new(),
            header_length: String::new(),
            type_of_service: String::new(),
            total_length: String::new(),
            identification: String::new(),
            flags: String::new(),
            fragment_offset: String::new(),
            time_to_live: String::new(),
            protocol: String::new(),
            header_checksum: String::new(),
            source_address: String::new(),
            destination_address: String::new(),
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
            options: String::new(),
            data: String::new(),
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
}

struct NetworkLayer;

impl NetworkLayer {

    pub fn new() -> Self {
        // TODO
        NetworkLayer {  }
    }

    pub fn send_packet(&self, packet: PingPacket) {
        println!("Sending packet: {:?}", packet);
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
