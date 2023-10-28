use reqwest::Error;
use serde::Serialize;

use super::binance;

#[derive(Debug, Serialize)]
pub enum SourceVariant {
    Binance,
    Coingecko,
    Coinmarketcap,
}

#[derive(Debug, Serialize)]
pub struct CryptoPrice {
    pub price: f64,
    pub currency: String,
    pub timestamp: u64,
    pub source: String,
}

pub trait CryptoPriceTracker {
    const NAME: &'static str;
}

impl SourceVariant {
    pub async fn get_price(&self, currencies: Vec<String>) -> Result<Vec<CryptoPrice>, Error> {
        match self {
            SourceVariant::Binance => binance::BinancePriceTracker::get_price(currencies).await,
            SourceVariant::Coingecko => todo!(),
            SourceVariant::Coinmarketcap => todo!(),
        }
    }
}
