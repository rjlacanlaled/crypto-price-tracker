enum SourceVariant {
    Binance,
    Coingecko,
    Coinmarketcap,
}

struct CryptoPrice {
    pub price: f64,
    pub currency: String,
    pub timestamp: u64,
    pub source: SourceVariant,
}

trait CryptoPriceTracker {
    fn get_current_price(&self, currency: &str) -> Result<CryptoPrice, Box<dyn std::error::Error>>;
}
