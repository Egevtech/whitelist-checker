mod check;
mod parser;

use check::check;
use clap::Parser;
use std::io::stdin;

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

#[derive(Parser)]
struct Args {
    /// Количество попыток подключения к серверам
    #[arg(long, short, default_value = "3")]
    tries: i32,

    /// Путь к файлу с серверами в белом списке
    #[arg(long, short, default_value = "./wl_servers.txt")]
    whitelisted: String,

    /// Путь к файлу с серверами вне белого списка
    #[arg(long, short, default_value = "./nwl_servers.txt")]
    not_whitelisted: String,
}

fn main() {
    let args = Args::parse();

    let white_url = vec!["yandex.com:443", "vk.com:443"];
    let not_white_url = vec!["google.com:443", "github.com:443"];

    let mut result: Res = Res::NoInternerConnection;

    for url in white_url {
        for i in 0..args.tries {
            if check(url.to_string(), i, args.tries) {
                result = Res::WhiteListEnabled;
            } else {
                result = Res::NoInternerConnection;
            };
        }
    }

    for url in not_white_url {
        for i in 0..args.tries {
            if check(url.to_string(), i, args.tries) {
                result = Res::FullInternetAvailable;
            }
        }
    }

    println!("\n===============Результат===============");
    println!("{result}");
    println!("=======================================");

    #[cfg(target_os = "windows")]
    {
        let mut stdin = Stdin;
        let mut s = "".to_string();

        println!("Нажмите Enter для выхода...");

        stdin.read_line(&mut s).expect("Ошибка чтения из stdin");
    }
}
