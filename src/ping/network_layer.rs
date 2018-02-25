use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use ping::PingPacket;

pub struct NetworkLayer {
    interface: datalink::NetworkInterface,
    transmitter: Box<datalink::DataLinkSender>,
    receiver: Box<datalink::DataLinkReceiver>,
}

impl NetworkLayer {

    pub fn new() -> Self {
        let interface = datalink::interfaces().first().unwrap().clone();
        let (mut tx, mut rx) = match datalink::channel(&interface, datalink::Config::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
        };
        NetworkLayer {
            interface: interface, // TODO this should be able to be dynamic
            transmitter: tx,
            receiver: rx,
        }
    }

    pub fn send_packet(&mut self, packet: PingPacket) {
        println!("Sending packet: {:?}", packet.into_bytes());
        match self.transmitter.send_to(&packet.into_bytes(), None) {
            None => { println!("Nothing sent"); } // TODO what does this really mean??
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
