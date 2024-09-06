use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::io;

pub struct Gateway {
    address: IpAddr,
    control_url: String,
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


        let mut lines = response.lines();
        lines.next();

        let mut headers = HashMap::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                headers.insert(key.to_string(), value.to_string());
            }
        }

        // Print extracted headers
        //for (key, value) in &headers {
        //    println!("{}: {}", key, value);
        //}

        println!("{}", headers.get("Location").unwrap());
        /*
        let self_ = Self {
            address,
            ..Default::default()
        };
        */

        //let data = std::str::from_utf8(data)?.trim().to_string();


        /*
        // Regex to find "Location" in the response
        let location_pattern = regex_find("(?i)Location:(.*)", &data);
        if let Some(url) = location_pattern {
            control_url = url.trim().to_string();
        }

        // Fetch XML data from the control URL
        let response = get_http_response(&control_url)?;

        // Regex to find serviceType and controlURL in the XML response
        let url_pattern = regex_find("(?i)<controlURL>(.*?)</controlURL>", &response);
        let service_type_pattern = regex_find("(?i)<serviceType>(.*?)</serviceType>", &response);

        let mut url_path = String::new();
        if let Some(service) = service_type_pattern {
            service_type = service;
        }
        if let Some(url) = url_pattern {
            url_path = url;
        }

        // Update the control URL
        control_url = build_control_url(&control_url, &url_path)?;
        */

        let control_url = String::new();
        let service_type = String::new();


        //Ok(self_)
        Ok(Self {
            address,
            control_url,
            service_type
        })
    }

    /*
        public GateWay(byte[] data, Inet4Address address)throws Exception {
            this.address = address;

            String response = new String(data).trim();
            Pattern pattern = Pattern.compile("Location:(?: |)(.*?)$", Pattern.DOTALL | Pattern.MULTILINE | Pattern.CASE_INSENSITIVE);

            Matcher matcher = pattern.matcher(response);
            while(matcher.find()){
                controlUrl = matcher.group(1);
            }

            pattern = Pattern.compile("(<controlURL>|<serviceType>)(.*?)(?:<\\/controlURL>|<\\/serviceType>)", Pattern.DOTALL | Pattern.MULTILINE | Pattern.CASE_INSENSITIVE);
            HttpURLConnection conn = (HttpURLConnection) new URL(controlUrl).openConnection();
            conn.setRequestMethod("GET");
            conn.setRequestProperty("Content-Type", "text/xml");

            byte[] buffer = new byte[8192];
            int length = conn.getInputStream().read(buffer);
            response = new String(buffer, 0, length);
            conn.disconnect();

            matcher = pattern.matcher(response);
            String urlPath = "";
            while(matcher.find()){
                if(matcher.group(1).equals("<serviceType>")){
                    serviceType = matcher.group(2);
                }else{
                    urlPath = matcher.group(2);
                }
            }

            try{
                URL url = new URL(controlUrl);
                controlUrl = url.getProtocol()+"://"+url.getHost()+":"+url.getPort()+urlPath;
            }catch(Exception e){
                throw new Exception("Couldn't parse url.");
            }
        }

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
