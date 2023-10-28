mod tracker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prices = tracker::base::SourceVariant::Binance.get_price(
        vec!["BTC".to_string(), "BNB".to_string()]
    ).await?;
    Ok(())
}
