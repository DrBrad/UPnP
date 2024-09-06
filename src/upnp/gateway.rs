use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, TcpStream};
use std::io;
use std::io::{Read, Write};
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
        println!("{}", response);

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

        //println!("Response:\n{}", response_str);



        let service_list_start = response_str.find("<serviceList>").unwrap();
        let service_list_content = &response_str[service_list_start..response_str[service_list_start..].find("</serviceList>").unwrap() + service_list_start];

        let service_start = service_list_content.find("<service>").unwrap();
        let service_content = &service_list_content[service_start..service_list_content[service_start..].find("</service>").unwrap() + service_start];

        let control_url_start = service_content.find("<controlURL>").unwrap() + "<controlURL>".len();
        let control_url = service_content[control_url_start..service_content[control_url_start..].find("</controlURL>").unwrap() + control_url_start].to_string();
        url.path = control_url;

        let service_type_start = service_content.find("<serviceType>").unwrap() + "<serviceType>".len();
        let service_type = service_content[service_type_start..service_content[service_type_start..].find("</serviceType>").unwrap() + service_type_start].to_string();


        println!("Control Url: {}", url.to_string());
        println!("Service Type: {}", service_type);

        //Ok(self_)
        Ok(Self {
            address,
            control_url: url,
            service_type
        })
    }

    pub fn get_external_ip(&self) -> IpAddr {
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
    }

    /*

        private HashMap<String, String> command(String action, Map<String, String> params)throws Exception {
            HashMap<String, String> ret = new HashMap<>();
            String soap = "<?xml version=\"1.0\"?>\r\n" +
                    "<SOAP-ENV:Envelope xmlns:SOAP-ENV=\"http://schemas.xmlsoap.org/soap/envelope/\" SOAP-ENV:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\">" +
                    "<SOAP-ENV:Body>" +
                    "<m:"+action+" xmlns:m=\""+serviceType+"\">";

            if(params != null){
                for(Map.Entry<String, String> entry : params.entrySet()){
                    soap += "<"+entry.getKey()+">"+entry.getValue()+"</" + entry.getKey()+">";
                }
            }
            soap += "</m:"+action+"></SOAP-ENV:Body></SOAP-ENV:Envelope>";
            byte[] req = soap.getBytes();
            HttpURLConnection conn = (HttpURLConnection) new URL(controlUrl).openConnection();
            conn.setRequestMethod("POST");
            conn.setDoOutput(true);
            conn.setRequestProperty("Content-Type", "text/xml");
            conn.setRequestProperty("SOAPAction", "\""+serviceType+"#"+action+"\"");
            conn.setRequestProperty("Connection", "Close");
            conn.setRequestProperty("Content-Length", ""+req.length);
            conn.getOutputStream().write(req);

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
        }

        public boolean openPort(int port, boolean udp){
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
        }

        public boolean closePort(int port, boolean udp){
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
        }

        public boolean isMapped(int port, boolean udp){
            if(port < 0 || port > 65535){
                throw new IllegalArgumentException("Invalid port");
            }
            HashMap<String, String> params = new HashMap<>();
            params.put("NewRemoteHost", "");
            params.put("NewProtocol", udp ? "UDP" : "TCP");
            params.put("NewExternalPort", ""+port);
            try{
                HashMap<String, String> ret = command("GetSpecificPortMappingEntry", params);
                if(ret.get("errorCode") != null){
                    throw new Exception();
                }
                return ret.get("NewInternalPort") != null;
            }catch(Exception e){
                return false;
            }
        }

        public String getExternalIP(){
            try{
                HashMap<String, String> ret = command("GetExternalIPAddress", null);
                return ret.get("NewExternalIPAddress");
            }catch(Exception e){
                return null;
            }
        }
    }
    */
}
