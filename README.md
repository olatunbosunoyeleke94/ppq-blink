# ppq-blink

A Rust crate for topping up ppq.ai (pay-per-query AI) credits using Blink Wallet's Lightning payments.

## Screenshot

![Top-Up using LN invoice](screenshots/<img width="1433" height="406" alt="Top-Up using LN invoice" src="https://github.com/user-attachments/assets/50e17bd1-0eb3-486f-8999-d6973f553765" />)

![Transaction History on PPQ](screenshots/<img width="760" height="277" alt="Transaction History on PPQ" src="https://github.com/user-attachments/assets/12140654-3a31-490e-89e9-797ae2a88e2d" />)

![Balance on PPQ homepage](screenshots/<img width="1433" height="667" alt="Balance in PPQ homepage" src="https://github.com/user-attachments/assets/d81a584f-e34d-46e6-93e5-e98728e69c71" />)


## Features

- Secure: API keys loaded from `.env` file
- Interactive: Prompts for Lightning invoice
- Simple: Pay top-ups with one command

## Installation

```bash
cargo add ppq-blink
