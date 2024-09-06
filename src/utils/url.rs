pub struct Url {
    pub(crate) scheme: String,
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) path: String
}

impl Url {

    pub fn new(url: &str) -> Self {
        let scheme_end = url.find("://").unwrap();
        let scheme = url[..scheme_end].to_string();

        let host_start = scheme_end + 3;
        let host_end = url[host_start..].find('/').unwrap_or_else(|| url.len() - host_start) + host_start;
        let host_port = &url[host_start..host_end];

        let (host, port) = if let Some(port_index) = host_port.find(':') {
            let host = host_port[..port_index].to_string();
            let port = host_port[port_index + 1..].parse::<u16>().unwrap_or(80);
            (host, port)
        } else {
            (host_port.to_string(), 80)
        };

        let path = url[host_end..].to_string();

        Self {
            scheme,
            host,
            port,
            path
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}://{}:{}{}", self.scheme, self.host, self.port, self.path)
    }
}
