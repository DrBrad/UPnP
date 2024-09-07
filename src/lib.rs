mod upnp;
mod utils;

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::{Read, Write};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, UdpSocket};
    use crate::upnp::protocol::Protocol::Udp;
    use crate::upnp::upnp::UPnP;

    #[test]
    fn is_mapped() {
        //println!("TEST");
        let upnp = UPnP::new(IpAddr::from([192, 168, 0, 129])).unwrap();
        //upnp.open_tcp_port(3030);
        //upnp.close_tcp_port(3030);
        println!("{} IP", upnp.get_external_ip().unwrap().to_string());
        println!("{:?} OPEN", upnp.is_mapped(27017, Udp).unwrap());
    }

    #[test]
    fn open_port() {

    }

}
