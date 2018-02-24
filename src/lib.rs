extern crate pnet;

mod ping;
use ping::{PingPacket, NetworkLayer};


pub fn execute() {
    let packet = PingPacket::new();
    let network_layer = NetworkLayer::new();
    network_layer.send_packet(packet);
    let echo_response = network_layer.listen_for_echo_response();
    println!("{:?}", echo_response);
}

