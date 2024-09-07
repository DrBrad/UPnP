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
        //println!("TEST");
        let upnp = UPnP::new().unwrap();
        //upnp.open_tcp_port(3030);
        //upnp.get_external_ip();
        upnp.is_tcp_mapped(27017);
    }
}
