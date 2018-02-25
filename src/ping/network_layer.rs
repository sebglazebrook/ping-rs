use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use ping::PingPacket;

pub struct NetworkLayer {
    interface: datalink::NetworkInterface,
}

impl NetworkLayer {

    pub fn new() -> Self {
        NetworkLayer {
            interface: datalink::interfaces().first().unwrap().clone(), // TODO this should be able to be dynamic
        }
    }

    pub fn send_packet(&self, packet: PingPacket) {
        println!("Sending packet: {:?}", packet.into_bytes());
        // TODO below is just me hacking
        let (mut tx, mut rx) = match datalink::channel(&self.interface, datalink::Config::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
        };
        match tx.send_to(&packet.into_bytes(), None) {
            None => { println!("Nothing sent"); }
            Some(result) => {
                match result {
                    Err(error) => { println!("Error!: {:?}", error); }
                    Ok(result) => { println!("Success!: {:?}", result); }
                }
            }
        }
    }

    pub fn listen_for_echo_response(&self) -> PingPacket {
        println!("Listening for echo responses");
        // TODO
        PingPacket::new()
    }


}
