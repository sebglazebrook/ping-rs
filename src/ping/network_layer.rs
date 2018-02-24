use ping::PingPacket;

pub struct NetworkLayer;

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
