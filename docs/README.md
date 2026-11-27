
# SolanaTokenManager

## About Project

SolanaTokenManager — програмний застосунок на Rust для створення та управління SPL-токенами в блокчейні Solana.

Проєкт призначений для автоматизації основних операцій із токенами: створення, ініціалізації, випуску та переказу.

## Features

- Підключення до Solana RPC.
- Генерація криптографічної ключової пари.
- Створення Mint Account.
- Ініціалізація SPL-токена.
- Отримання адреси Associated Token Account.
- Випуск токенів.
- Підготовка переказу токенів.
- Можливість майбутньої інтеграції з Pump.fun.

## Technologies

- Rust 2024
- Solana SDK
- Solana RPC Client
- SPL Token
- Cargo

## Project Structure

```text
my_project/
├── docs/
│   ├── README.md
│   ├── specification.md
│   ├── business_logic.md
│   ├── architecture.md
│   └── project_policy.md
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── .gitignore
```

## Documentation

| File | Description |
|---|---|
| `README.md` | Загальна інформація про проєкт. |
| `specification.md` | Специфікація системи. |
| `business_logic.md` | Опис бізнес-логіки. |
| `architecture.md` | Архітектура системи. |
| `project_policy.md` | Політика проєкту. |

## Requirements

Для роботи із системою необхідно:

- Встановлений Rust.
- Встановлений Cargo.
- Доступ до мережі Інтернет.
- Доступ до Solana RPC.
- Налаштовані залежності проєкту.

## Installation

1. Клонувати репозиторій.
2. Перейти до папки проєкту.
3. Встановити необхідні залежності.
4. Перевірити конфігурацію.
5. Запустити застосунок.

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

## Security Notice

Програма повинна використовувати захищене зберігання приватних ключів.

Не використовуйте реальні кошти під час тестування неперевіреного коду.

Перед запуском транзакцій у Mainnet необхідно перевірити параметри операцій.

## Current Status

Проєкт перебуває на початковому етапі розробки.

Базові операції створення та управління SPL-токенами описані у вихідному коді. Інтеграція з Pump.fun, автоматизоване тестування та повноцінна обробка помилок потребують подальшої реалізації.

## Version

0.1.0
