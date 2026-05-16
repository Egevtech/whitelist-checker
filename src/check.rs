use ping;
use std::{net::ToSocketAddrs, time::Duration};

pub fn check(url: String, iteration: i32, repeats: i32) -> bool {
    let address = match url.to_socket_addrs() {
        Ok(mut ok) => ok.next().unwrap().ip(),
        Err(err) => {
            eprintln!(
                "[{}/{repeats}] Неудачный пинг {url} \nОшибка {}",
                iteration + 1,
                err.to_string()
            );
            return false;
        }
    };

    match ping::new(address)
        .timeout(Duration::from_secs(2))
        .ttl(128)
        .send()
    {
        Ok(ok) => {
            println!(
                "[{}/{repeats}] Пинг {url} отклик={}мс",
                iteration + 1,
                ok.rtt.as_millis().to_string()
            );
            return true;
        }
        Err(e) => {
            eprintln!(
                "[{}/{repeats}] Неудачный пинг {url} \nОшибка {e}",
                iteration + 1,
            );
            return false;
        }
    };
}
