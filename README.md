# ppq-blink

A Rust crate for topping up ppq.ai (pay-per-query AI) credits using Blink Wallet's Lightning payments.

## Features

- Secure: API keys loaded from `.env` file
- Interactive: Prompts for Lightning invoice
- Simple: Pay top-ups with one command

## Installation

```bash
cargo add ppq-blink

## Usage

Create .env in your project:

PPQ_API_KEY=ppq_sk_...
BLINK_API_KEY=bk_...
BLINK_WALLET_ID=your_btc_wallet_id

## Run

```bash
cargo run

Paste your ppq.ai Lightning top-up invoice when prompted.
