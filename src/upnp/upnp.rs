use std::io;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use crate::upnp::gateway::Gateway;
use crate::upnp::protocol::Protocol;

const REQUESTS: [&str; 3] = [
    "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nST: urn:schemas-upnp-org:device:InternetGatewayDevice:1\r\nMAN: \"ssdp:discover\"\r\nMX: 2\r\n\r\n",
    "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nST: urn:schemas-upnp-org:service:WANIPConnection:1\r\nMAN: \"ssdp:discover\"\r\nMX: 2\r\n\r\n",
    "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nST: urn:schemas-upnp-org:service:WANPPPConnection:1\r\nMAN: \"ssdp:discover\"\r\nMX: 2\r\n\r\n"
];

pub struct UPnP {
    gateway: Option<Gateway>
}

impl UPnP {

    pub fn new(local_addr: IpAddr) -> io::Result<Self> {
        let local = SocketAddr::new(local_addr, 0);
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

    pub fn open_port(&self, port: u16, protocol: Protocol) -> io::Result<bool> {
        self.gateway.as_ref().unwrap().open_port(port, protocol)
    }

    pub fn close_port(&self, port: u16, protocol: Protocol) -> io::Result<bool> {
        self.gateway.as_ref().unwrap().close_port(port, protocol)
    }

    pub fn is_mapped(&self, port: u16, protocol: Protocol) -> io::Result<bool> {
        self.gateway.as_ref().unwrap().is_mapped(port, protocol)
    }

    pub fn get_external_ip(&self) -> io::Result<IpAddr> {
        self.gateway.as_ref().unwrap().get_external_ip()
    }
}
