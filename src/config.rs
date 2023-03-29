use dotenv::dotenv;
use std::net::Ipv4Addr;

pub fn from_env() -> (Ipv4Addr, u16) {
    // loads the environment variables from the ".env" file.
    dotenv().ok();
    // get listening port
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    // ensure port is valid type
    let port: u16 = port.parse().expect("Port should be valid range");

    // get host
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let host: Ipv4Addr = host.parse().expect("Not a valid IP address");
    // let host =
    (host, port)
}
