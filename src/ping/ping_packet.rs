use ping::IpDatagram;

#[derive(Debug)]
pub struct PingPacket {
    ip_datagram: IpDatagram,
}

impl PingPacket {

    pub fn new() -> Self {
        PingPacket {
            ip_datagram: IpDatagram::new(), // TODO create an IP packet with the correct IP header rather than hardcoding everything
        }
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        self.ip_datagram.into_bytes()
    }
}
