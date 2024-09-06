mod upnp;

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
    use crate::upnp::upnp::UPnP;

    #[test]
    fn open_port() {
        //let socket = UdpSocket::bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED, 0));

        let upnp = UPnP::new();

    }
}
