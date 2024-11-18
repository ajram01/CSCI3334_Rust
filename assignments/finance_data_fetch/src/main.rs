use dotenv::dotenv;
use std::env;
use serde::Deserialize;
use serde_json::json;
use ureq;
use std::fs::File;
use std::io::Write;

// Struct to store API address and output file name for Bitcoin pricing data.
struct Bitcoin {
    btc_api_address: String,
    file_name: String,
}

// Struct to store API address and output file name for Ethereum pricing data.
struct Ethereum {
    eth_api_address: String,
    file_name: String,
}

// Struct to store API address and output file name for S&P 500 pricing data.
struct Sp500 {
    sp500_api_address: String,
    file_name: String,
}

// Trait to standardize methods for fetching and saving pricing data.
pub trait Pricing {
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self);
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> f32 {
        // Fetch the latest Bitcoin price from the API.
        // Return 0.0 if the API call or JSON parsing fails.
        let url = format!("{}", self.btc_api_address);
        let btc_response = ureq::get(&url).call();

        if let Ok(btc_response) = btc_response {
            if let Ok(body) = btc_response.into_string() {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                    if let Some(price) = json["bitcoin"]["usd"].as_f64() {
                        return price as f32;
                    }
                }
            }
        }
        0.0
    }

    fn save_to_file(&self) {
        let price = self.fetch_price();
        let json_data = json!({
            "bitcoin": { "usd": price }
        });

        // Save the fetched price to a JSON file.
        if let Ok(mut file) = File::create(&self.file_name) {
            serde_json::to_writer_pretty(file, &json_data).expect("Failed to write to file");
            println!("Saved price to file {}", &self.file_name);
        } else {
            println!("Failed to write to file {}", &self.file_name);
        }
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> f32 {
        // Fetch the latest Ethereum price from the API.
        // Return 0.0 if the API call or JSON parsing fails.
        let url = format!("{}", self.eth_api_address);
        let eth_response = ureq::get(&url).call();

        if let Ok(eth_response) = eth_response {
            if let Ok(body) = eth_response.into_string() {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                    if let Some(price) = json["ethereum"]["usd"].as_f64() {
                        return price as f32;
                    }
                }
            }
        }
        0.0
    }

    fn save_to_file(&self) {
        let price = self.fetch_price();
        let json_data = json!({
            "ethereum": { "usd": price }
        });

        // Save the fetched price to a JSON file.
        if let Ok(mut file) = File::create(&self.file_name) {
            serde_json::to_writer_pretty(file, &json_data).expect("Failed to write to file");
            println!("Saved price to file {}", &self.file_name);
        } else {
            println!("Failed to write to file {}", &self.file_name);
        }
    }
}

impl Pricing for Sp500 {
    fn fetch_price(&self) -> f32 {
        // Fetch the latest S&P 500 price from the API.
        // Return 0.0 if the API call or JSON parsing fails.
        let url = format!("{}", self.sp500_api_address);
        let sp500_response = ureq::get(&url).call();

        if let Ok(sp500_response) = sp500_response {
            if let Ok(body) = sp500_response.into_string() {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                    if let Some(price) = json["vanguard-sp-500-etf-tokenized-stock-defichain"]["usd"].as_f64() {
                        return price as f32;
                    }
                }
            }
        }
        0.0
    }

    fn save_to_file(&self) {
        let price = self.fetch_price();
        let json_data = json!({
            "SP500": { "usd": price }
        });

        // Save the fetched price to a JSON file.
        if let Ok(mut file) = File::create(&self.file_name) {
            serde_json::to_writer_pretty(file, &json_data).expect("Failed to write to file");
            println!("Saved price to file {}", &self.file_name);
        } else {
            println!("Failed to write to file {}", &self.file_name);
        }
    }
}



fn main() {
    // Load environment variables and fetch API endpoints for Bitcoin, Ethereum, and S&P 500.
    dotenv().ok();
    let btc_api_address = env::var("BITCOIN_API_ENDPOINT").expect("BITCOIN_API_ENDPOINT not set in .env file");
    let eth_api_address = env::var("ETHEREUM_API_ENDPOINT").expect("ETHEREUM_API_ENDPOINT not set in .env file");
    let sp500_api_address = env::var("SP500_API_ENDPOINT").expect("SP500_API_ENDPOINT not set in .env file");

    // Initialize instances of each struct.
    let crypto: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { btc_api_address, file_name: "btc_prices.json".to_string() }),
        Box::new(Ethereum { eth_api_address, file_name: "eth_prices.json".to_string() }),
        Box::new(Sp500 { sp500_api_address, file_name: "sp500_prices.json".to_string() }),
    ];

    // Periodically save pricing data for each asset every 10 seconds.
    loop {
        for coin in &crypto {
            coin.save_to_file();
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
