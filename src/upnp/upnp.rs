use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use crate::upnp::gateway;
use crate::upnp::gateway::Gateway;

const REQUESTS: [&str; 3] = [
    "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nST: urn:schemas-upnp-org:device:InternetGatewayDevice:1\r\nMAN: \"ssdp:discover\"\r\nMX: 2\r\n\r\n",
    "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nST: urn:schemas-upnp-org:service:WANIPConnection:1\r\nMAN: \"ssdp:discover\"\r\nMX: 2\r\n\r\n",
    "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nST: urn:schemas-upnp-org:service:WANPPPConnection:1\r\nMAN: \"ssdp:discover\"\r\nMX: 2\r\n\r\n"
];

pub struct UPnP {
    gateway: Option<Gateway>
}

impl UPnP {

    pub fn new() -> io::Result<Self> {
        //let local = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0));
        let local = SocketAddr::new(IpAddr::from([192, 168, 0, 129]), 0);
        let socket = UdpSocket::bind(local)?;

        for req in REQUESTS {
            let address = SocketAddr::new(IpAddr::from([239, 255, 255, 250]), 1900);
            socket.send_to(req.as_bytes(), address)?;

            let mut buf = [0; 1536];
            match socket.recv_from(&mut buf) {
                Ok((size, src_addr)) => {
                    let gateway = Gateway::new(&buf, size, local.ip())?;

                    return Ok(Self {
                        gateway: Some(gateway)
                    });
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => {
                    break;
                }
            }
        }

        Err(io::Error::new(io::ErrorKind::Other, "Could not find gateway"))
    }

    pub fn open_tcp_port(&self, port: u16) {
        //self.gateway.open_port(port, false)
    }

    pub fn close_tcp_port(&self, port: u16) {
        //self.gateway.close_port(port, false)
    }

    pub fn open_udp_port(&self, port: u16) {
        //self.gateway.open_port(port, true)
    }

    pub fn close_udp_port(&self, port: u16) {
        //self.gateway.close_port(port, true)
    }

    pub fn get_external_ip(&self) -> IpAddr {
        self.gateway.as_ref().unwrap().get_external_ip()
    }

    pub fn is_tcp_mapped(&self, port: u16) {
        //self.gateway.is_mapped(port, false)
    }

    pub fn is_udp_mapped(&self, port: u16) {
        //self.gateway.is_mapped(port, true)
    }
}
