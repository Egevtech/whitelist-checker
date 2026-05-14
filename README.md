# Whitelist-Checker

Утилита для проверки состояния белых списков в РФ.

## Использование
1. Скачать исполняемый файл для вашей системы со страницы [релизов](https://github.com/MikhKuts/whitelist-checker/releases)
2. Запустить либо двойным кликом, либо через консоль
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