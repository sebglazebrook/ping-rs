// TODO future -> handle things other than IPv4
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
}

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

}

#[derive(Debug)]
pub struct PingPacket {
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
