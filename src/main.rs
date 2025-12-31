use ppq_blink::{PpqBlinkClient, PpqBlinkError};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ppq-blink: ppq.ai + Blink Wallet Lightning Payments");

    // Load .env file automatically
    dotenvy::dotenv().ok();

    // Read credentials from environment (populated from .env)
    let ppq_api_key = std::env::var("PPQ_API_KEY")
        .expect("PPQ_API_KEY not set (check your .env file)");

    let blink_api_key = std::env::var("BLINK_API_KEY")
        .expect("BLINK_API_KEY not set (check your .env file)");

    let blink_wallet_id = std::env::var("BLINK_WALLET_ID")
        .expect("BLINK_WALLET_ID not set (check your .env file)");

    let client = PpqBlinkClient::new(ppq_api_key, blink_api_key, blink_wallet_id);

    println!("✅ Client ready!");

    // Prompt user to paste the invoice
    print!("📋 Paste your ppq.ai Lightning invoice (lnbc...): ");
    io::stdout().flush()?;

    let mut bolt11_invoice = String::new();
    io::stdin()
        .read_line(&mut bolt11_invoice)
        .expect("Failed to read invoice");

    let bolt11_invoice = bolt11_invoice.trim();

    if bolt11_invoice.is_empty() || !bolt11_invoice.starts_with("lnbc") {
        eprintln!("❌ Invalid or empty invoice. Exiting.");
        return Ok(());
    }

    println!("💸 Paying invoice ({} chars)...", bolt11_invoice.len());

    match client.pay_lightning_invoice(bolt11_invoice).await {
        Ok(status) => {
            println!("✅ Payment completed! Status: {}", status);
            if status == "SUCCESS" {
                println!("🎉 Credits should appear in your ppq.ai dashboard shortly!");
            } else {
                println!("ℹ️  Status: {} (may be PENDING or ALREADY_PAID)", status);
            }
        }
        Err(e) => {
            eprintln!("❌ Payment failed: {:?}", e);
            if let PpqBlinkError::Api(msg) = e {
                println!("🔍 API error details:\n{}", msg);
            }
        }
    }

    Ok(())
}
