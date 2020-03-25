use ra_common::models::{Network, Packet};

use log::{trace,info};

pub struct VPNClient {

}

impl VPNClient {
    pub fn new() -> Box<VPNClient> {
        Box::new(VPNClient {

        })
    }
    pub fn init(&mut self) {
        info!("{}","Initializing VPN Client...")

    }
}

impl Network for VPNClient {
    fn handle(&mut self, packet: &mut Packet) {
        info!("Handling incoming packet id={}",packet.id);

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
