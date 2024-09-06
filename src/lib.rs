mod upnp;
mod utils;

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::{Read, Write};
    use std::net::{Ipv4Addr, SocketAddr, TcpStream, UdpSocket};
    use crate::upnp::upnp::UPnP;

    #[test]
    fn open_port() {
        let upnp = UPnP::new();
    }
}
