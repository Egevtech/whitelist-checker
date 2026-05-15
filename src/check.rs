use ping;
use std::{net::ToSocketAddrs, time::Duration};

pub fn check(url: String, iteration: i32, repeats: i32) -> bool {
    let addres = match url.to_socket_addrs() {
        Ok(mut ok) => ok.next().unwrap().ip(),
        Err(err) => {
            eprintln!(
                "[{}/{}] Неудачный пинг {} \nОшибка {}",
                iteration + 1,
                repeats,
                url,
                err.to_string()
            );
            return false;
        }
    };

    match ping::new(addres)
        .timeout(Duration::from_secs(2))
        .ttl(128)
        .send()
    {
        Ok(ok) => {
            println!(
                "[{}/{}] Пинг {} отклик={}мс",
                iteration + 1,
                repeats,
                url,
                ok.rtt.as_millis().to_string()
            );
            return true;
        }
        Err(e) => {
            eprintln!(
                "[{}/{}] Неудачный пинг {} \nОшибка {}",
                iteration + 1,
                repeats,
                url,
                e
            );
            return false;
        }
    };
}
