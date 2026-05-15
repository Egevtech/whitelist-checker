use std::{fmt::write, io::stdin, net::ToSocketAddrs, path::Display, time::Duration, vec};

enum Res {
    NoInternerConnection,
    WhiteListEnabled,
    FullInternetAvailable,
}

impl std::fmt::Display for Res {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Res::NoInternerConnection => write!(f, "Нет интернет соединения"),
            Res::WhiteListEnabled => write!(f, "Белые списки включены"),
            Res::FullInternetAvailable => write!(f, "Интернет не ограничен"),
        }
    }
}

fn main() {
    let white_url = vec!["yandex.com:443", "vk.com:443"];
    let not_white_url = vec!["google.com:443", "github.com:443"];
    let mut result: Res = Res::NoInternerConnection;

    let repeats = 3;
    for url in white_url {
        for i in 0..repeats {
            if ping(url.to_string(), i, repeats) {
                result = Res::WhiteListEnabled;
            } else {
                result = Res::NoInternerConnection;
            };
        }
    }
    for url in not_white_url {
        for i in 0..repeats {
            if ping(url.to_string(), i, repeats) {
                result = Res::FullInternetAvailable;
            }
        }
    }

    let stdin = stdin();

    println!("\n===============Результат===============");
    println!("{result}");
    println!("=======================================");

    let mut s = "".to_string();
    println!("Нажмите Enter для выхода...");
    stdin.read_line(&mut s).expect("Ошибка чтения из stdin");
}

fn ping(url: String, iteration: i32, repeats: i32) -> bool {
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
