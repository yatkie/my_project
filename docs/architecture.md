
# SolanaTokenManager ARCHITECTURE

> Архітектурний опис програмної системи для створення та управління SPL-токенами в блокчейні Solana. Документ визначає основні компоненти системи, їхню відповідальність, взаємодію між собою та зовнішніми сервісами.

## 1. System Overview

`SolanaTokenManager` — програмний застосунок, написаний мовою Rust, призначений для виконання операцій зі створення, ініціалізації, випуску та переказу SPL-токенів у мережі Solana.

Система взаємодіє з блокчейном через Solana RPC Client та використовує бібліотеки Solana SDK і SPL Token.

### Main Technologies

| Technology | Purpose |
|---|---|
| Rust | Основна мова програмування. |
| Cargo | Управління залежностями та складанням проєкту. |
| Solana SDK | Робота з ключами, адресами та транзакціями. |
| Solana Client | Взаємодія з RPC-сервером. |
| SPL Token | Створення та управління токенами. |
| Associated Token Account | Отримання адрес токен-акаунтів. |
| Solana Blockchain | Виконання та зберігання транзакцій. |

## 2. Architectural Style

Для подальшого розвитку системи передбачається використання модульної багатошарової архітектури.

Основні архітектурні шари:

- Domain Layer — бізнес-правила та доменні об'єкти.
- Application Layer — організація бізнес-процесів.
- Infrastructure Layer — взаємодія з Solana RPC та зовнішніми сервісами.
- Presentation Layer — взаємодія з користувачем, якщо буде доданий інтерфейс.

> Поточна реалізація не має повністю розділених архітектурних шарів. Основна логіка розміщена у файлі `src/main.rs`. Наведена нижче структура описує цільову архітектуру розвитку системи.

## 3. System Components

| Component | Responsibility |
|---|---|
| `main.rs` | Точка входу програми та послідовний запуск операцій. |
| `RpcClient` | Підключення до Solana RPC. |
| `Wallet` | Генерація ключової пари та підписання транзакцій. |
| `TokenMint` | Створення та ініціалізація токена. |
| `TokenAccount` | Визначення адреси Associated Token Account. |
| `MintService` | Організація операції випуску токенів. |
| `TransferService` | Організація переказу токенів. |
| `PumpFunIntegration` | Майбутня інтеграція з Pump.fun. |

> Назви сервісів та компонентів, які відсутні у поточному коді, є логічним поділом цільової архітектури, а не реалізованими Rust-модулями.

## 4. Component Diagram

Основна схема взаємодії компонентів:

```text
              USER
                |
                v
           main.rs
                |
                v
          APPLICATION
                |
       +--------+--------+
       |        |        |
       v        v        v
    Wallet   Token    Transfer
             Service  Service
       |        |        |
       +--------+--------+
                |
                v
          INFRASTRUCTURE
                |
          +-----+------+
          |            |
          v            v
      Solana RPC    SPL Token
          |            |
          +-----+------+
                |
                v
         SOLANA BLOCKCHAIN
                |
                v
         Transaction Result
```

> Схема відображає цільовий логічний поділ. У поточній реалізації виклики виконуються безпосередньо з `main.rs`.

## 5. Domain Layer

Domain Layer відповідає за предметну область управління токенами.

Основні доменні елементи:

### Entities

- `TokenMint` — представлення створеного токена.
- `TokenAccount` — представлення токен-акаунта.
- `Wallet` — представлення гаманця.
- `TokenTransfer` — представлення переказу токенів.

### Value Objects

- `Pubkey` — публічний ключ Solana.
- `Keypair` — пара криптографічних ключів.
- `TokenAmount` — кількість токенів.
- `TokenDecimals` — кількість десяткових знаків.

### Business Rules

- Перевірка коректності адрес.
- Контроль кількості токенів.
- Перевірка прав Mint Authority.
- Контроль параметрів переказу.
- Захист приватних ключів.

## 6. Application Layer

Application Layer відповідає за організацію основних операцій системи.

Основні сценарії:

| Use Case | Description |
|---|---|
| `CreateToken` | Створення нового токена. |
| `InitializeToken` | Ініціалізація Mint Account. |
| `MintTokens` | Випуск початкової кількості токенів. |
| `TransferTokens` | Переказ токенів іншому власнику. |
| `ListToken` | Майбутня інтеграція з Pump.fun. |

Application Layer повинен координувати виконання операцій та передавати запити до відповідних доменних та інфраструктурних компонентів.

## 7. Infrastructure Layer

Infrastructure Layer забезпечує взаємодію програми із зовнішніми системами.

### 7.1 Solana RPC Client

Відповідає за:
- Підключення до RPC-сервера.
- Отримання мінімального балансу для створення акаунтів.
- Відправлення транзакцій.
- Отримання підтверджень транзакцій.

Поточна адреса RPC:

`https://api.mainnet-beta.solana.com`

### 7.2 Solana SDK

Використовується для:
- Генерації ключових пар.
- Роботи з публічними ключами.
- Формування транзакцій.
- Підписання операцій.

### 7.3 SPL Token Program

Відповідає за:
- Ініціалізацію Mint Account.
- Випуск токенів.
- Переказ токенів.
- Виконання інструкцій SPL Token.

### 7.4 Associated Token Account

Використовується для визначення адреси токен-акаунта конкретного власника та токена.

### 7.5 External Integrations

У майбутньому передбачається інтеграція з Pump.fun для автоматизації процесу лістингу токенів.

На поточному етапі інтеграція не реалізована.

## 8. Data Flow

### 8.1 Token Creation Flow

```text
START
  |
  v
Initialize RPC Client
  |
  v
Generate Wallet
  |
  v
Generate Mint Keypair
  |
  v
Calculate Rent Exemption
  |
  v
Create Mint Account
  |
  v
Initialize SPL Token
  |
  v
Return Token Address
  |
  v
END
```

### 8.2 Token Minting Flow

```text
START
  |
  v
Get Token Account Address
  |
  v
Prepare Mint Instruction
  |
  v
Sign Transaction
  |
  v
Send Transaction
  |
  v
Verify Result
  |
  v
END
```

### 8.3 Token Transfer Flow

```text
START
  |
  v
Get Sender Token Account
  |
  v
Validate Recipient Address
  |
  v
Set Transfer Amount
  |
  v
Prepare Transfer Instruction
  |
  v
Sign Transaction
  |
  v
Send Transaction
  |
  v
Verify Result
  |
  v
END
```

## 9. Security Architecture

Безпека системи забезпечується на рівні гаманців, транзакцій та мережевої взаємодії.

### Wallet Security

- Використання криптографічних ключових пар.
- Захищене зберігання приватних ключів.
- Заборона публікації секретних даних.
- Контроль доступу до операцій підписання.

### Transaction Security

- Перевірка адрес одержувачів.
- Перевірка кількості токенів.
- Контроль повноважень підписанта.
- Перевірка результатів транзакцій.

### Network Security

- Використання HTTPS для RPC-з'єднання.
- Контроль доступності RPC-сервера.
- Обробка помилок мережевої взаємодії.

## 10. Error Handling Architecture

Система повинна передбачати обробку помилок на кожному рівні взаємодії.

| Error Category | Responsible Component |
|---|---|
| RPC Connection Error | Infrastructure |
| Wallet Generation Error | Wallet |
| Mint Creation Error | Token Service |
| Initialization Error | Token Service |
| Minting Error | Mint Service |
| Transfer Error | Transfer Service |
| Invalid Address | Domain / Application |

Для обробки помилок рекомендується використовувати тип `Result<T, E>` мови Rust.

## 11. Deployment Architecture

Поточна система є консольним застосунком.

Основні елементи середовища:

- Операційна система Windows або Linux.
- Встановлене середовище Rust.
- Cargo для складання програми.
- Доступ до мережі Інтернет.
- Доступ до Solana RPC.

### Execution Flow

1. Користувач запускає програму.
2. Rust-компілятор створює виконуваний файл.
3. Програма встановлює RPC-з'єднання.
4. Виконуються операції з токенами.
5. Результати операцій виводяться у консоль.

## 12. Architecture Limitations

Поточна реалізація має такі обмеження:

- Основна логіка розміщена в одному файлі.
- Відсутній окремий Domain Layer.
- Відсутній окремий Application Layer.
- Відсутня централізована обробка помилок.
- Відсутній окремий механізм логування.
- Відсутні автоматизовані інтеграційні тести.
- Інтеграція з Pump.fun не реалізована.
- Відсутня конфігурація через окремий файл.

## 13. Future Architecture Improvements

Для подальшого розвитку системи передбачається:

- Розділення коду на окремі модулі.
- Створення сервісів для управління токенами.
- Впровадження Domain Policies.
- Реалізація повноцінної обробки помилок.
- Додавання логування.
- Реалізація автоматизованого тестування.
- Безпечне управління гаманцями.
- Інтеграція з Pump.fun.

---

**Document:** architecture.md

**Project:** SolanaTokenManager

**Version:** 0.1.0

**Architecture:** Modular Layered Architecture (Target)

**Language:** Rust

**Blockchain:** Solana
