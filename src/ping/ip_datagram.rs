use ping::{IpHeader, IcmpPacket};

#[derive(Debug)]
pub struct IpDatagram {
    header: IpHeader,
    options: String, // 32 bit
    data: IcmpPacket, // what's the bits??
}

impl IpDatagram {

    pub fn new() -> Self {
        IpDatagram {
            header: IpHeader::new(),
            options: String::new(), // TODO as the name says, these are optional
            data: IcmpPacket::new(),
        }
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let mut payload = vec![];
        payload.extend(self.header.into_bytes());
        payload.extend(self.options.clone().into_bytes());
        payload.extend(self.data.into_bytes());
        payload
    }

}
