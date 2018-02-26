use ping::IpHeader;
// First iteration, make one successful ping request with hardcoded destination and payload

#[derive(Debug)]
struct IcmpHeader {
    // first 32 bits
    message_type: u8, // just `type` in rfc
    code: u8,
    checksum: u16,
    // second 32 bits
    identifier: u16,
    sequence_number: u16,
}

impl IcmpHeader {

    pub fn new() -> Self {
        IcmpHeader {
            message_type: 8, // 8 = echo, 0 = echo reply
            code: 0, // don't know why just is in the rfc
            checksum: 16, // TODO calculate this properly
            identifier: 0, // TODO this should be the process id of the sending process
            sequence_number: 0, // TODO increments with each echo/echo reply (is this the same as identifier??)
        }
    }

}

#[derive(Debug)]
struct IcmpPacket {
    header: IcmpHeader, // 8 bytes
    payload: String, // optional ( can store send time in here and then calculate the trip time when it's returned )
}

impl IcmpPacket {

    pub fn new() -> Self {
        IcmpPacket {
            header: IcmpHeader::new(),
            payload: String::new(),
        }
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        vec![] // TODO return actual bytes
    }
}

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
