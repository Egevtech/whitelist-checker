use std::{error::Error, net::ToSocketAddrs, time::Duration};




fn main() {
    let white_url = vec![
        "yandex.com:443",
        "vk.com:443",
    ];
    let not_white_url = vec![
        "google.com:443",
        "github.com:443"
    ];

    for url in white_url {
        ping(url.to_string());
        
    }

    
}

fn ping(url: String){
    let addres = url.to_socket_addrs().unwrap().next().unwrap().ip();

    match ping::new(addres)
        .timeout(Duration::from_secs(2))
        .ttl(128)
        .send()
    {
        Ok(_) => println!("Ping successful with custom options!"),
        Err(e) => eprintln!("Ping failed: {}", e),
    }

}


