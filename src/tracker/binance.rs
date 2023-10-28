use reqwest::{ Error, Client };
use serde::{ Serialize, Deserialize };
use serde_this_or_that::as_f64;
use chrono::{ Local, DateTime };

use super::base::{ CryptoPriceTracker, CryptoPrice };
use crate::tracker::base::SourceVariant;

#[derive(Debug, Deserialize, Serialize)]
pub struct BinancePriceResponse {
    pub symbol: String,
    #[serde(deserialize_with = "as_f64")]
    pub price: f64,
}

pub struct BinancePriceTracker;

impl CryptoPriceTracker for BinancePriceTracker {
    const NAME: &'static str = "Binance";
}

impl BinancePriceTracker {
    pub async fn get_price(currencies: Vec<String>) -> Result<Vec<CryptoPrice>, Error> {
        let currencies = currencies.into_iter().map(|c| format!("{}{}", c.to_uppercase(), "USDT"));
        println!("{:?}", currencies);
        let client = Client::new();
        let res = client
            .get("https://api.binance.com/api/v3/ticker/price")
            .query(
                &[
                    (
                        "symbols",
                        currencies
                            .into_iter()
                            .reduce(|a, b| format!("\"{}\",\"{}\"", a, b))
                            .map(|s| format!("[{}]", s))
                            .unwrap(),
                    ),
                ]
            )
            .send().await?
            .json::<Vec<BinancePriceResponse>>().await?;

        let res = res
            .into_iter()
            .map(|r| CryptoPrice {
                price: r.price,
                currency: r.symbol,
                timestamp: chrono::Utc::now().timestamp() as u64,
                source: "Binance".to_string(),
            })
            .collect();

        println!("{:#?}", res);
        Ok(res)
    }
}
