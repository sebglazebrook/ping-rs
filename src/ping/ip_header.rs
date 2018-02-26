use byteorder::{WriteBytesExt, BigEndian};
// TODO future -> handle things other than IPv4

// TODO the IpHeader is big endian
#[derive(Debug)]
pub struct IpHeader {
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
            destination_address: String::from("192.160.0.69").into_bytes(), // TODO this should be configurable (currently google)
        }

    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let mut payload = vec![];
        payload.push(self.version);
        payload.push(self.header_length);
        payload.push(self.type_of_service);

        // Converting total_length from u16 -> Vec<u8>
        let mut total_length: Vec<u8> = Vec::new();
        total_length.write_u16::<BigEndian>(self.total_length).unwrap();
        payload.extend(total_length);

        // Converting identification from u16 -> Vec<u8>
        let mut identification: Vec<u8> = Vec::new();
        identification.write_u16::<BigEndian>(self.identification).unwrap();
        payload.extend(identification);

        payload.push(self.flags);
        payload.push(self.fragment_offset);
        payload.push(self.time_to_live);
        payload.push(self.protocol);

        // Converting header_checksum from u16 -> Vec<u8>
        let mut header_checksum: Vec<u8> = Vec::new();
        header_checksum.write_u16::<BigEndian>(self.header_checksum);
        payload.extend(header_checksum);

        payload.extend(self.source_address.clone());
        payload.extend(self.destination_address.clone());
        payload
    }
}
