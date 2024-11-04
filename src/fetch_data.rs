use std::error::Error;
use reqwest::blocking::Client;
use polars::prelude::*;
use serde_json::Value;

pub fn get_basic_market_info(url: &str) -> Result<DataFrame, Box<dyn Error>> {
   let client = Client::new();
   let response = client.get(url).send()?;
   
   if response.status().as_u16() != 200 {
       return Err("Non-200 response".into());
   }

   let json: Value = response.json()?;
   let event_type = &json["eventTypes"][0];
   let event_node = &event_type["eventNodes"][0];
   let market_node = &event_node["marketNodes"][0];

   
   let event_name = event_node["event"]["eventName"]
       .as_str()
       .unwrap_or("unknown")
       .to_string();

    let event_id = event_node["eventId"]
       .as_i64()
       .unwrap_or(0);
    
   let market_name = market_node["description"]["marketName"]
       .as_str()
       .unwrap_or("unknown")
       .to_string();

    let market_id: String = market_node["marketId"]
       .as_str()
       .unwrap_or("unknown")
       .to_string();

    let market_total_matched = market_node["state"]["totalMatched"]
       .as_f64()
       .unwrap_or(0.0);

    let market_total_available = market_node["state"]["totalAvailable"]
         .as_f64()
         .unwrap_or(0.0);

   let df = DataFrame::new(vec![
        Series::new("url", vec![url]),
        Series::new("event_name", vec![event_name]),
        Series::new("event_id", vec![event_id]),
        Series::new("market_name", vec![market_name]),
        Series::new("market_id", vec![market_id]),
        Series::new("market_total_matched", vec![market_total_matched]),
        Series::new("market_total_available", vec![market_total_available]),
   ])?;

   Ok(df)
}

pub fn get_selections_info(url: &str) -> Result<DataFrame, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send()?;
    
    if response.status().as_u16() != 200 {
        return Err("Non-200 response".into());
    }

    let json: Value = response.json()?;
    let market_node = &json["eventTypes"][0]["eventNodes"][0]["marketNodes"][0];
    let market_id = market_node["marketId"].as_str().unwrap_or("unknown");
    
    let mut selection_names = Vec::new();
    let mut selection_ids = Vec::new();
    let mut selection_handicaps = Vec::new();
    let mut selection_prices = Vec::new();
    let mut selection_sizes = Vec::new();
    let mut selection_back_or_lays = Vec::new();
    let mut market_ids = Vec::new();

    for runner in market_node["runners"].as_array().unwrap() {
        // Back prices
        if let Some(backs) = runner["exchange"]["availableToBack"].as_array() {
            for back in backs {
                selection_names.push(runner["description"]["runnerName"].as_str().unwrap_or("unknown").to_string());
                selection_ids.push(runner["selectionId"].as_i64().unwrap_or(0));
                selection_handicaps.push(runner["handicap"].as_f64().unwrap_or(0.0));
                selection_prices.push(back["price"].as_f64().unwrap_or(0.0));
                selection_sizes.push(back["size"].as_f64().unwrap_or(0.0));
                selection_back_or_lays.push("back".to_string());
                market_ids.push(market_id.to_string());
            }
        }

        // Lay prices
        if let Some(lays) = runner["exchange"]["availableToLay"].as_array() {
            for lay in lays {
                selection_names.push(runner["description"]["runnerName"].as_str().unwrap_or("unknown").to_string());
                selection_ids.push(runner["selectionId"].as_i64().unwrap_or(0));
                selection_handicaps.push(runner["handicap"].as_f64().unwrap_or(0.0));
                selection_prices.push(lay["price"].as_f64().unwrap_or(0.0));
                selection_sizes.push(lay["size"].as_f64().unwrap_or(0.0));
                selection_back_or_lays.push("lay".to_string());
                market_ids.push(market_id.to_string());
            }
        }
    }

    let df = DataFrame::new(vec![
        Series::new("market_id", market_ids),
        Series::new("selection_name", selection_names),
        Series::new("selection_id", selection_ids),
        Series::new("selection_handicap", selection_handicaps),
        Series::new("selection_price", selection_prices),
        Series::new("selection_size", selection_sizes),
        Series::new("selection_back_or_lay", selection_back_or_lays),
    ])?;

    Ok(df)
}

pub fn get_market_data(url: &str) -> Result<DataFrame, Box<dyn Error>> {
    let market_info = get_basic_market_info(url)?;
    let selections = get_selections_info(url)?;
    
    let joined = market_info.join(
        &selections,
        ["market_id"],
        ["market_id"],
        JoinType::Inner.into(),
    )?;

    Ok(joined)
}
