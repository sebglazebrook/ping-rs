use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use ping::PingPacket;

pub struct NetworkLayer;

impl NetworkLayer {

    pub fn new() -> Self {
        // TODO find the network interface we're going to use
        NetworkLayer {  }
    }

    pub fn send_packet(&self, packet: PingPacket) {

        println!("Sending packet: {:?}", packet.into_bytes());
        // TODO below is just me hacking
        let interfaces = datalink::interfaces();
        let interface = interfaces.first().unwrap();
        let (mut tx, mut rx) = match datalink::channel(&interface, datalink::Config::default()) {
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
