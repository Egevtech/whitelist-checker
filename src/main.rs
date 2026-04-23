use std::{error::Error, io::{Read, stdin}, net::ToSocketAddrs, time::Duration};




fn main() {
    let white_url = vec![
        "yandex.com:443",
        "vk.com:443",
    ];
    let not_white_url = vec![
        "google.com:443",
        "github.com:443"
    ];

    for url in [white_url, not_white_url].concat() {
        for i in 0..3 {
            ping(url.to_string());
        }
        
    }

    let stdin = stdin();

    let mut s = "".to_string();
    println!("Нажмите Enter для выхода...");
    stdin.read_line(&mut s);
}

fn ping(url: String){
    let addres = match url.to_socket_addrs() {
        Ok(mut ok) => ok.next().unwrap().ip(),
        Err(err) => {
            eprintln!("Ошибка {}", err.to_string());
            return;
        }
    };
    

    match ping::new(addres)
        .timeout(Duration::from_secs(2))
        .ttl(128)
        .send()
    {
        Ok(ok) => println!("Ping {} ping={}ms", url, ok.rtt.as_millis().to_string()),
        Err(e) => eprintln!("Ping failed: {}", e),
    }

}


