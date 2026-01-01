# ppq-blink

A Rust crate for topping up ppq.ai (pay-per-query AI) credits using Blink Wallet's Lightning payments.

## Screenshot

![ppq.ai Balance on Homepage](screenshots/balance-in-ppq-homepage.png)

![ppq.ai Top-Up with Lightning Invoice](screenshots/top-up-using-ln-invoice.png)

![ppq.ai Transaction History](screenshots/transaction-history-on-ppq.png)

## Features

- Secure: API keys loaded from `.env` file
- Interactive: Prompts for Lightning invoice
- Simple: Pay top-ups with one command

## Installation

```bash
cargo add ppq-blink
