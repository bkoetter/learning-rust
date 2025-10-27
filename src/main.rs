use hmac::{Hmac, KeyInit, Mac};
use reqwest::{blocking::Client, Method, Url};
use serde::Serialize;
use sha2::Sha256;
use std::env::var;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
struct Signature {
    timestamp: u128,
    method: Method,
    api_url: Url,
    api_key: String,
    api_secret: String,
    signature: Option<String>,
}

impl Signature {
    fn new(url: Url) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            method: Method::GET,
            api_url: url,
            api_key: var("API_KEY").unwrap(),
            api_secret: var("API_SECRET").unwrap(),
            signature: None,
        }
    }
    fn get_signature(mut self) -> Self {
        let path = match self.api_url.query() {
            Some(query) => format!("{}?{}", self.api_url.path(), query),
            None => self.api_url.path().to_string(),
        };
        let mut mac = HmacSha256::new_from_slice(self.api_secret.as_bytes()).unwrap();
        mac.update(&format!("{}{}{}", self.timestamp, self.method, path).into_bytes());
        self.signature = Some(
            mac.finalize()
                .into_bytes()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect(),
        );
        self
    }
}

#[derive(serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct History {
    pub items: Vec<HistoryItem>,
    pub current_page: i64,
    pub total_pages: i64,
    pub max_items: i64,
}

#[derive(serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct HistoryItem {
    pub transaction_id: String,
    pub executed_at: String,
    pub r#type: String,
    pub price_currency: Option<String>,
    pub price_amount: Option<String>,
    pub sent_currency: Option<String>,
    pub sent_amount: Option<String>,
    pub received_currency: String,
    pub received_amount: String,
    pub fees_currency: Option<String>,
    pub fees_amount: Option<String>,
    pub address: Option<String>,
}

fn get_transaction_history(config: &Signature) -> reqwest::Result<History> {
    let url = config.api_url.as_str();
    Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("Bitvavo-Access-Key", config.api_key.as_str())
        .header("Bitvavo-Access-Timestamp", config.timestamp.to_string())
        .header(
            "Bitvavo-Access-Signature",
            config.signature.as_ref().unwrap(),
        )
        .send()?
        .json::<History>()
}

fn history_print(history: Vec<HistoryItem>) {
    let currency = "BTC";
    let x = history
        .into_iter()
        .filter(|item| item.r#type == "buy" && item.received_currency == currency)
        .map(|item| {
            println!("{}: {} {}", item.executed_at, item.received_amount, item.price_amount.as_ref().unwrap());
            (
                item.received_amount.parse::<f64>().unwrap()
                    * item.price_amount.as_ref().unwrap().parse::<f64>().unwrap(),
                item.received_amount.parse::<f64>().unwrap(),
            )
        })
        .reduce(|acc, x| (acc.0 + x.0, acc.1 + x.1));
    if let Some((total_spent, total_btc)) = x {
        println!("Total spent on {currency}: {:.2} EUR", total_spent);
        println!("Total {currency} bought: {:.8} BTC", total_btc);
        println!("Average price: {:.2} EUR/{currency}", total_spent / total_btc);
    } else {
        println!("No {currency} buy transactions found.");
    }
}

fn main() {
    let url = Url::parse("https://api.bitvavo.com/v2/account/history").unwrap();
    let signature = Signature::new(url).get_signature();
    if let Ok(history) = get_transaction_history(&signature) {
        history_print(history.items);
        println!("{}", history.total_pages);
    } else {
        eprintln!("Failed to fetch history");
    }
}
