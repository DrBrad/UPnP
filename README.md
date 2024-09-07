UPnP
=====

This is a Rust implementation of UPnP as a library. You can easily add this to your project.

Implementation
-----
Below are some of the commands you can use:

```rust
let upnp = UPnP::new(IpAddr::from([192, 168, 0, 129])).expect("Cannot find gateway");
println!("{}", upnp.get_external_ip().unwrap().to_string());
println!("OPEN: {:?}", upnp.open_port(4040, Tcp).unwrap());
println!("MAPPED: {:?}", upnp.is_mapped(4040, Tcp).unwrap());
println!("CLOSE: {:?}", upnp.close_port(4040, Tcp).unwrap());

//FOR UDP
println!("MAPPED: {:?}", upnp.is_mapped(4040, Udp).unwrap());
```
