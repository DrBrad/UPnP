use std::collections::HashMap;
use std::net::{IpAddr, TcpStream};
use std::io;
use std::io::{Read, Write};
use std::str::FromStr;
use crate::upnp::protocol::Protocol;
use crate::utils::url::Url;

pub struct Gateway {
    address: IpAddr,
    control_url: Url,
    service_type: String
}

impl Gateway {

    pub fn new(buf: &[u8], size: usize, address: IpAddr) -> io::Result<Self> {
        let response = std::str::from_utf8(&buf[..size]).unwrap_or("[Invalid UTF-8]");

        let mut lines = response.lines();
        lines.next();

        let mut headers = HashMap::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        let mut url = Url::new(headers.get("Location").or_else(|| headers.get("LOCATION")).unwrap());


        let mut stream = TcpStream::connect((url.host.clone(), url.port.clone())).unwrap();

        let request = format!("GET {} HTTP/1.1\r\n\
                   Host: {}\r\n\
                   Content-Type: text/xml\r\n\r\n", url.path.clone(), url.host.clone());
        stream.write_all(request.as_bytes()).unwrap();

        let mut response = Vec::new();
        let mut reader = io::BufReader::new(stream);
        reader.read_to_end(&mut response).unwrap();

        let response_str = String::from_utf8_lossy(&response);

        let doc = roxmltree::Document::parse(response_str.split("\r\n\r\n").last().unwrap())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let root = doc.root_element().descendants().find(|node| node.tag_name().name() == "device").unwrap()
            .descendants().find(|node| node.tag_name().name() == "deviceList").unwrap()
            .descendants().find(|node| node.tag_name().name() == "device").unwrap()
            .descendants().find(|node| node.tag_name().name() == "deviceList").unwrap()
            .descendants().find(|node| node.tag_name().name() == "device").unwrap()
            .descendants().find(|node| node.tag_name().name() == "serviceList").unwrap()
            .descendants().find(|node| node.tag_name().name() == "service").unwrap();

        url.path = root.descendants().find(|node| node.tag_name().name() == "controlURL").unwrap().text().unwrap().to_string();
        let service_type = root.descendants().find(|node| node.tag_name().name() == "serviceType").unwrap().text().unwrap().to_string();

        Ok(Self {
            address,
            control_url: url,
            service_type
        })
    }

    pub fn open_port(&self, port: u16, protocol: Protocol) -> io::Result<bool> {
        let mut params = HashMap::new();
        params.insert("NewRemoteHost".to_string(), "".to_string());
        params.insert("NewProtocol".to_string(), protocol.value().to_string());
        params.insert("NewInternalClient".to_string(), self.address.to_string());
        params.insert("NewExternalPort".to_string(), port.to_string());
        params.insert("NewInternalPort".to_string(), port.to_string());
        params.insert("NewEnabled".to_string(), "1".to_string());
        params.insert("NewPortMappingDescription".to_string(), "UPnP".to_string());
        params.insert("NewLeaseDuration".to_string(), "0".to_string());

        let response = self.command("AddPortMapping", Some(params));

        match response {
            Ok(_) => {
                Ok(true)
            }
            Err(_) => {
                Ok(false)
            }
        }
    }

    pub fn close_port(&self, port: u16, protocol: Protocol) -> io::Result<bool> {
        let mut params = HashMap::new();
        params.insert("NewRemoteHost".to_string(), "".to_string());
        params.insert("NewProtocol".to_string(), protocol.value().to_string());
        params.insert("NewExternalPort".to_string(), port.to_string());

        let response = self.command("DeletePortMapping", Some(params));

        match response {
            Ok(_) => {
                Ok(true)
            }
            Err(_) => {
                Ok(false)
            }
        }
    }

    pub fn is_mapped(&self, port: u16, protocol: Protocol) -> io::Result<bool> {
        let mut params = HashMap::new();
        params.insert("NewRemoteHost".to_string(), "".to_string());
        params.insert("NewProtocol".to_string(), protocol.value().to_string());
        params.insert("NewExternalPort".to_string(), port.to_string());

        let response = self.command("GetSpecificPortMappingEntry", Some(params));

        match response {
            Ok(map) => {
                Ok(map.get("NewEnabled").unwrap() == "1")
            }
            Err(_) => {
                Ok(false)
            }
        }
    }

    pub fn get_external_ip(&self) -> io::Result<IpAddr> {
        let response = self.command("GetExternalIPAddress", None)?;
        Ok(IpAddr::from_str(response.get("NewExternalIPAddress").unwrap()).unwrap())
    }

    fn command(&self, action: &str, params: Option<HashMap<String, String>>) -> io::Result<HashMap<String, String>> {
        let mut soap = format!("<?xml version=\"1.0\"?>\r\n\
            <SOAP-ENV:Envelope xmlns:SOAP-ENV=\"http://schemas.xmlsoap.org/soap/envelope/\" SOAP-ENV:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\">\
            <SOAP-ENV:Body>\
            <m:{} xmlns:m=\"{}\">", action, self.service_type);

        match params {
            Some(params) => {
                if !params.is_empty() {
                    for (key, value) in params.iter() {
                        soap.push_str(format!("<{}>{}</m{}>", key, value, key).as_str());
                    }
                }
            }
            None => {}
        }

        soap.push_str(format!("</m:{}></SOAP-ENV:Body></SOAP-ENV:Envelope>", action).as_str());


        let mut stream = TcpStream::connect((self.control_url.host.clone(), self.control_url.port.clone())).unwrap();

        let request = format!("POST {} HTTP/1.1\r\n\
                   Host: {}\r\n\
                   Content-Type: text/xml\r\n\
                   SOAPAction: \"{}#{}\"\r\n\
                   Content-Length: {}\r\n\r\n", self.control_url.path.clone(), self.control_url.host.clone(), self.service_type, action, soap.as_bytes().len());

        stream.write_all(request.as_bytes()).unwrap();
        stream.write_all(soap.as_bytes()).unwrap();

        let mut response = Vec::new();
        let mut reader = io::BufReader::new(stream);
        reader.read_to_end(&mut response).unwrap();

        let response_str = String::from_utf8_lossy(&response);

        let doc = roxmltree::Document::parse(response_str.split("\r\n\r\n").last().unwrap())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        if let Some(root) = doc.root_element().descendants().find(|node| node.tag_name().name() == "Body").unwrap()
                .descendants().find(|node| node.tag_name().name() == format!("{}Response", action)) {
            let mut response = HashMap::new();

            let mut iter = root.descendants();
            iter.next();
            while let Some(node) = &iter.next() {
                if node.is_element() {
                    let key = node.tag_name().name().to_string();
                    if let Some(node) = iter.next() {
                        if node.is_text() {
                            response.insert(key, node.text().unwrap().to_string());
                        }
                    }
                }
            }

            return Ok(response);
        }

        let error_code = doc.descendants().find(|node| node.tag_name().name() == "errorCode").unwrap().text().unwrap();
        let error_description = doc.descendants().find(|node| node.tag_name().name() == "errorDescription").unwrap().text().unwrap();

        Err(io::Error::new(io::ErrorKind::Other, format!("{}: {}", error_code, error_description)))
    }
}
