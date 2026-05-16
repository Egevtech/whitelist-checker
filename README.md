# Whitelist-Checker

Утилита для проверки состояния белых списков в РФ или наличия интернет соединения в целом.

## Использование
1. Скачать исполняемый файл для вашей системы со страницы [релизов](https://github.com/MikhKuts/whitelist-checker/releases)
2. Запустить либо двойным кликом, либо через консоль:

```shell
whitelist-checker.exe [OPTIONS]

Options:
  -t, --tries <TRIES>
          Количество попыток подключения к серверам [default: 3]
  -w, --whitelisted <WHITELISTED>
          Путь к файлу с серверами в белом списке [default: ./wl_servers.txt]
  -n, --not-whitelisted <NOT_WHITELISTED>
          Путь к файлу с серверами вне белого списка [default: ./nwl_servers.txt]
  -h, --help
          Print help
  ```

## Сборка из исходников
Для этого понадобятся 
- Cargo
- Компилятор для Rust

Если все есть, то вводим в консоль:
```shell
git clone https://github.com/MikhKuts/whitelist-checker.git
cd whitelist-checker
cargo build --release
```
Скомпилированный файл будет находиться по пути `whitelist-checker\target\release`

## Как работает
1. Отправляет пинг на 4 сайта (2 находящихся в белом списке, 2 нет) по 3 раза (исключает последствия тайм-аута)
2. На основе успешности пингов выводит результат:
  - "Нет интернет соединения"
  - "Белые списки включены"
  - "Интернет не ограничен"
