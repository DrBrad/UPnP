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
        //let socket = UdpSocket::bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED, 0));

        let upnp = UPnP::new();




        /*
        let control_url = "http://192.168.8.1:5555/rootDesc.xml";
        let host = "192.168.8.1";
        let port = 5555;
        let path = "/rootDesc.xml";


        let mut stream = TcpStream::connect((host, port)).unwrap();

        // Send the HTTP GET request
        let request = format!("GET {} HTTP/1.1\r\n\
                   Host: {}\r\n\
                   Content-Type: text/xml\r\n\r\n", path, host);
        stream.write_all(request.as_bytes()).unwrap();

        // Read the response
        let mut response = Vec::new();
        let mut reader = io::BufReader::new(stream);
        reader.read_to_end(&mut response).unwrap();

        // Convert response to a string
        let response_str = String::from_utf8_lossy(&response);

        // Print the response for debugging
        println!("Response:\n{}", response_str);
        */

    }
}
