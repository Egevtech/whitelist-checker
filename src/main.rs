mod check;

use check::check;
use clap::Parser;
use serde::Deserialize;
use serde_yaml;

const DEFAULT_CONFIG: &str = include_str!("../wc_config.yaml");
const CONFIG_PATH: &str = "~/.config/wc_config.yaml";

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
    /// Завершать проверку домена при успешном пинге
    #[arg(short, long)]
    successful_skip: bool,

    /// Количество попыток подключения к серверам
    #[arg(long, short)]
    tries: Option<u32>,

    /// Путь к файлу с серверами в белом списке
    #[arg(long, short, default_value = "./wl_servers.txt")]
    whitelisted: String,

    /// Путь к файлу с серверами вне белого списка
    #[arg(long, short, default_value = "./nwl_servers.txt")]
    not_whitelisted: String,

    /// Восстановить дефолтную конфигурацию
    #[arg(long, short)]
    restore: bool,

    /// Уровень дебага
    #[arg(short, action=clap::ArgAction::Count)]
    debug: u8,
}

fn check_urls(urls: Vec<String>, tries: u32) -> bool {
    let mut result: bool = true;

    for url in urls {
        for i in 0..tries {
            print!("Попытка {}/{} пинга {url}...", i + 1, tries);
            match check(url.clone()) {
                Ok(k) => match k {
                    Ok(rtt) => {
                        println!(" успех, отклик={rtt}мс");
                        break;
                    }
                    Err(e) => {
                        println!();
                        eprintln!("Ошибка: {e}");
                    }
                },
                Err((msg, err)) => {
                    println!();
                    eprintln!("{msg}: {err}");
                }
            }
            result = false;
        }
    }
    result
}

#[derive(Deserialize)]
struct Config {
    whitelisted: Vec<String>,
    not_whitelisted: Vec<String>,
    tries: u8,
}

// TODO: rename to autorestore()
fn check_config() -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Starting autorestore");
    if std::fs::exists(CONFIG_PATH)? {
        if std::path::Path::new(CONFIG_PATH).is_dir() {
            log::error!("Fatal: config path is an directory.");
            std::process::exit(-1);
        }
        log::info!("No autorestore needed");
        return Ok(());
    }

    autorestore()
}

// TODO: rename to restore()
fn autorestore() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Restoring config...");

    std::fs::create_dir_all("~/.config/whitelist-checker/")?;
    std::fs::write("~/.config/whitelist-checker/wc-config.yaml", DEFAULT_CONFIG)?;

    log::info!("Autorestore completed");

    Ok(())
}

fn main() {
    compile_error!("Under construction");

    println!("Whitelist Checker v{}", env!("CARGO_PKG_VERSION"));
    let args = Args::parse();

    env_logger::builder()
        .filter_level(match args.debug {
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            _ => log::LevelFilter::Debug,
        })
        .init();

    if let Err(e) = check_config() {
        log::error!("Autorestore failed: {e}");
        std::process::exit(-1);
    }

    let config: Config =
        serde_yaml::from_str(&std::fs::read_to_string(CONFIG_PATH).expect("Failed to read config"))
            .expect("Failed to parse config");

    let mut result: Res = Res::NoInternerConnection;

    _ = result;

    println!("Проверяем сервера из белого списка:");

    println!("===============Результат===============");
    println!("{result}");
    println!("=======================================");

    #[cfg(target_os = "windows")]
    {
        use std::io::stdin;

        let stdin = stdin();
        let mut s = "".to_string();

        println!("Нажмите Enter для выхода...");

        stdin.read_line(&mut s).expect("Ошибка чтения из stdin");
    }
}
