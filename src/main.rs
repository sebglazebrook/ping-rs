// First iteration, make one successful ping request with hardcoded destination and payload

#[derive(Debug)]
struct PingPacket;

impl PingPacket {

    pub fn new() -> Self {
        // TODO
        PingPacket {}
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
        PingPacket {  }
    }
}

fn main() {
    let packet = PingPacket::new();
    let network_layer = NetworkLayer::new();
    network_layer.send_packet(packet);
    let echo_response = network_layer.listen_for_echo_response();
    println!("{:?}", echo_response);
}
