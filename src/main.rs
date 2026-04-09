use std::time::Duration;

use ping_rs::{PingError, PingReply};



fn main() {
    match request("77.88.44.242".to_owned()) {
        Ok(ok) => println!("Ping {} time={}ms", ok.address, ok.rtt),
        Err(err) => println!("Ping error"),
    }
}

fn request(address: String) -> Result<PingReply, PingError> {
    let addr = &address.parse().unwrap();
    let data = [1,2,3,4];  // ping data
    let timeout = Duration::from_secs(1);
    let options = ping_rs::PingOptions { ttl: 128, dont_fragment: true };
    return ping_rs::send_ping(addr, timeout, &data, Some(&options));
}