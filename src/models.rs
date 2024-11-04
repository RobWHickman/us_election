use serde::{Deserialize};

#[derive(Debug)]
pub struct MarketRow {
    pub url: String,
    pub event_name: String,
    pub event_id: i64,
    pub market_name: String,
    pub market_id: String,
    pub market_total_matched: f64,
    pub market_total_available: f64,
    pub selection_name: String,
    pub selection_id: i64,
    pub selection_handicap: f64,
    pub selection_price: f64,
    pub selection_size: f64,
    pub selection_back_or_lay: String,
}

#[derive(Debug, Deserialize)]
pub struct BetfairResponse {
    pub event_types: Vec<EventType>,
}

#[derive(Debug, Deserialize)]
pub struct EventType {
    #[serde(rename = "eventNodes")]
    pub event_nodes: Vec<EventNode>,
}

#[derive(Debug, Deserialize)]
pub struct EventNode {
    #[serde(rename = "eventId")]
    pub event_id: i64,
    pub event: Event,
    #[serde(rename = "marketNodes")]
    pub market_nodes: Vec<MarketNode>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "eventName")]
    pub event_name: String,
}

#[derive(Debug, Deserialize)]
pub struct MarketNode {
    #[serde(rename = "marketId")]
    pub market_id: String,
    pub description: MarketDescription,
    pub state: MarketState,
    pub runners: Vec<Runner>,
}

#[derive(Debug, Deserialize)]
pub struct MarketDescription {
    #[serde(rename = "marketName")]
    pub market_name: String,
}

#[derive(Debug, Deserialize)]
pub struct MarketState {
    #[serde(rename = "totalMatched")]
    pub total_matched: f64,
    #[serde(rename = "totalAvailable")]
    pub total_available: f64,
}

#[derive(Debug, Deserialize)]
pub struct Runner {
    #[serde(rename = "selectionId")]
    pub selection_id: i64,
    pub handicap: f64,
    pub description: RunnerDescription,
    pub exchange: Exchange,
}

#[derive(Debug, Deserialize)]
pub struct RunnerDescription {
    #[serde(rename = "runnerName")]
    pub runner_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Exchange {
    #[serde(rename = "availableToBack")]
    pub available_to_back: Option<Vec<Price>>,
    #[serde(rename = "availableToLay")]
    pub available_to_lay: Option<Vec<Price>>,
}

#[derive(Debug, Deserialize)]
pub struct Price {
    pub price: f64,
    pub size: f64,
}