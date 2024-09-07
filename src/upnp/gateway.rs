use std::collections::HashMap;
use std::fmt::format;
use std::net::{IpAddr, Ipv4Addr, TcpStream};
use std::io;
use std::io::{Read, Write};
//use crate::utils::ordered_map::OrderedMap;
use crate::utils::url::Url;

pub struct Gateway {
    address: IpAddr,
    control_url: Url,
    service_type: String
}

/*
impl Default for Gateway {

    fn default() -> Self {
        Self {
            address: Ipv4Addr::UNSPECIFIED,
            control_url: String::new(),
            service_type: String::new()
        }
    }
}
*/


impl Gateway {

    pub fn new(buf: &[u8], size: usize, address: IpAddr) -> io::Result<Self> {
        let response = std::str::from_utf8(&buf[..size]).unwrap_or("[Invalid UTF-8]");
        //println!("{}", response);

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

        println!("Location: {}", url.to_string());



        let mut stream = TcpStream::connect((url.host.clone(), url.port.clone())).unwrap();

        let request = format!("GET {} HTTP/1.1\r\n\
                   Host: {}\r\n\
                   Content-Type: text/xml\r\n\r\n", url.path.clone(), url.host.clone());
        stream.write_all(request.as_bytes()).unwrap();

        let mut response = Vec::new();
        let mut reader = io::BufReader::new(stream);
        reader.read_to_end(&mut response).unwrap();

        let response_str = String::from_utf8_lossy(&response);


        let device_list_start = response_str.find("<deviceList>").unwrap();
        let device_list_content = &response_str[device_list_start..response_str[device_list_start..].rfind("</deviceList>").unwrap() + device_list_start];

        let device_start = device_list_content.find("<device>").unwrap();
        let device_content = &device_list_content[device_start..device_list_content[device_start..].rfind("</device>").unwrap() + device_start];

        let device_list_start = device_content.find("<deviceList>").unwrap();
        let device_list_content = &device_content[device_list_start..device_content[device_list_start..].find("</deviceList>").unwrap() + device_list_start];

        let device_start = device_list_content.find("<device>").unwrap();
        let device_content = &device_list_content[device_start..device_list_content[device_start..].find("</device>").unwrap() + device_start];

        let service_list_start = device_content.find("<serviceList>").unwrap();
        let service_list_content = &device_content[service_list_start..device_content[service_list_start..].find("</serviceList>").unwrap() + service_list_start];

        let service_start = service_list_content.find("<service>").unwrap();
        let service_content = &service_list_content[service_start..service_list_content[service_start..].find("</service>").unwrap() + service_start];

        let control_url_start = service_content.find("<controlURL>").unwrap() + "<controlURL>".len();
        let control_url = service_content[control_url_start..service_content[control_url_start..].find("</controlURL>").unwrap() + control_url_start].to_string();
        url.path = control_url;

        let service_type_start = service_content.find("<serviceType>").unwrap() + "<serviceType>".len();
        let service_type = service_content[service_type_start..service_content[service_type_start..].find("</serviceType>").unwrap() + service_type_start].to_string();

        println!("Control Url: {}", url.to_string());
        println!("Service Type: {}", service_type);

        Ok(Self {
            address,
            control_url: url,
            service_type
        })
    }

    pub fn open_port(&self, port: u16) {
        if port > 65535 {

        }

        let mut params = HashMap::new();
        params.insert("NewRemoteHost".to_string(), "".to_string());
        params.insert("NewProtocol".to_string(), "TCP".to_string());
        params.insert("NewInternalClient".to_string(), self.address.to_string());
        params.insert("NewExternalPort".to_string(), port.to_string());
        params.insert("NewInternalPort".to_string(), port.to_string());
        params.insert("NewEnabled".to_string(), "1".to_string());
        params.insert("NewPortMappingDescription".to_string(), "UPnP".to_string());
        params.insert("NewLeaseDuration".to_string(), "0".to_string());

        let response = self.command("AddPortMapping", Some(params));

        /*
        if(port < 0 || port > 65535){
            throw new IllegalArgumentException("Invalid port");
        }
        HashMap<String, String> params = new HashMap<>();
        params.put("NewRemoteHost", "");
        params.put("NewProtocol", udp ? "UDP" : "TCP");
        params.put("NewInternalClient", address.getHostAddress());
        params.put("NewExternalPort", port+"");
        params.put("NewInternalPort", port+"");
        params.put("NewEnabled", "1");
        params.put("NewPortMappingDescription", "UNet");
        params.put("NewLeaseDuration", "0");
        try{
            HashMap<String, String> ret = command("AddPortMapping", params);
            return ret.get("errorCode") == null;
        }catch(Exception e){
            return false;
        }
        */
    }

    pub fn close_port(&self, port: u16) {
        /*
        if(port < 0 || port > 65535){
            throw new IllegalArgumentException("Invalid port");
        }
        HashMap<String, String> params = new HashMap<>();
        params.put("NewRemoteHost", "");
        params.put("NewProtocol", udp ? "UDP" : "TCP");
        params.put("NewExternalPort", ""+port);
        try{
            command("DeletePortMapping", params);
            return true;
        }catch(Exception e){
            return false;
        }
        */
    }

    pub fn is_mapped(&self, port: u16) {
        if port > 65535 {

        }

        let mut params = HashMap::new();
        params.insert("NewProtocol".to_string(), "TCP".to_string());
        params.insert("NewExternalPort".to_string(), port.to_string());
        params.insert("NewRemoteHost".to_string(), "".to_string());

        let response = self.command("GetSpecificPortMappingEntry", Some(params));
        /*
        if(port < 0 || port > 65535){
            throw new IllegalArgumentException("Invalid port");
        }
        try{
            HashMap<String, String> ret = command("GetSpecificPortMappingEntry", params);
            if(ret.get("errorCode") != null){
                throw new Exception();
            }
            return ret.get("NewInternalPort") != null;
        }catch(Exception e){
            return false;
        }
        */
    }

    pub fn get_external_ip(&self) -> IpAddr {
        let response = self.command("GetExternalIPAddress", None);
        //response.get("NewExternalIPAddress")
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
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
                        soap.push_str(format!("<m{}>{}</m{}>", key, value, key).as_str());
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
        println!("{}", request);
        println!("{}", soap);
        stream.write_all(request.as_bytes()).unwrap();
        stream.write_all(soap.as_bytes()).unwrap();

        let mut response = Vec::new();
        let mut reader = io::BufReader::new(stream);
        reader.read_to_end(&mut response).unwrap();

        let response_str = String::from_utf8_lossy(&response);
        println!("Response:\n{}", response_str);

        let response = HashMap::new();



        Ok(response)
        /*
        Document document = DocumentBuilderFactory.newInstance().newDocumentBuilder().parse(conn.getInputStream());
        NodeIterator iterator = ((DocumentTraversal) document).createNodeIterator(document.getDocumentElement(), NodeFilter.SHOW_ELEMENT, null, true);
        Node node;
        while((node = iterator.nextNode()) != null){
            try{
                if(node.getFirstChild().getNodeType() == Node.TEXT_NODE){
                    ret.put(node.getNodeName(), node.getTextContent());
                }
            }catch(Exception e){
                e.printStackTrace();
            }
        }
        conn.disconnect();
        return ret;
        */
    }
}
