use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::error::Error;
use std::env;

fn read_market_config(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let json_data: HashMap<String, String> = serde_json::from_str(&file_content)?;
    Ok(json_data)
}

#[derive(Debug, Deserialize)]
struct Config {
    base_url: String,
    by_market: String,
    r#return: String,  // 'return' is a keyword, so we use r#return
    uk_localisation: String,
    roll_up: String,
    market_types: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            base_url: env::var("BASE_URL").expect("BASE_URL not set"),
            by_market: env::var("BY_MARKET").expect("BY_MARKET not set"),
            r#return: env::var("RETURN").expect("RETURN not set"),
            uk_localisation: env::var("UK_LOCALISATION").expect("UK_LOCALISATION not set"),
            roll_up: env::var("ROLL_UP").expect("ROLL_UP not set"),
            market_types: env::var("MARKET_TYPES").expect("MARKET_TYPES not set"),
        }
    }
}

fn url_creator(market_id: &str, config: &Config) -> String {
    let request: Vec<String> = vec![
        format!("alt={}", config.r#return),
        config.uk_localisation.clone(),
        format!("marketIds={}", market_id),
        config.roll_up.clone(),
        config.market_types.clone(),
    ];
    
    format!("{}{}&{}", 
        config.base_url,
        config.by_market,
        request.join("&")
    )
}

pub fn get_market_urls() -> Result<Vec<String>, Box<dyn Error>> {
    let election_market_path = "./data/betfair_outcome_markets.json";
    let state_market_path = "./data/betfair_state_markets.json";
 
    let election_markets = read_market_config(election_market_path)?;
    let state_markets = read_market_config(state_market_path)?;
    
    let config = Config::from_env();
 
    let mut all_urls = Vec::new();
    
    for (_, market_id) in election_markets.iter() {
        all_urls.push(url_creator(&market_id.to_string(), &config));
    }
    
    for (_, market_id) in state_markets.iter() {
        all_urls.push(url_creator(&market_id.to_string(), &config));
    }
 
    Ok(all_urls)
}
