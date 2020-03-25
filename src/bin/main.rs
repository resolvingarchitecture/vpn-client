extern crate log;
extern crate simple_logger;

use log::{trace,info};

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting VPN Client Daemon...");
    let mut client = vpn_client::VPNClient::new();
    client.init();
    trace!("VPN Client Daemon Stopped.");
}
