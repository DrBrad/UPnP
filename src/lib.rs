mod upnp;
mod utils;

#[cfg(test)]
mod tests {

    use std::net::IpAddr;
    use crate::upnp::protocol::Protocol::Tcp;
    use crate::upnp::upnp::UPnP;

    #[test]
    fn test() {
        let upnp = UPnP::new(IpAddr::from([192, 168, 0, 129])).expect("Cannot find gateway");
        println!("{}", upnp.get_external_ip().unwrap().to_string());
        println!("OPEN: {:?}", upnp.open_port(4040, Tcp).unwrap());
        println!("MAPPED: {:?}", upnp.is_mapped(4040, Tcp).unwrap());
        println!("CLOSE: {:?}", upnp.close_port(4040, Tcp).unwrap());
    }
}
