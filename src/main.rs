use std::error::Error;
use std::thread;
use std::time::Duration;

mod hashmap_markets;
mod models;
mod fetch_data;
mod output;

fn main() -> Result<(), Box<dyn Error>> {
    let urls = hashmap_markets::get_market_urls()?;
    
    let mut all_dfs = Vec::new();
    
    for url in urls.iter() {
        match fetch_data::get_market_data(url) {
            Ok(df) => {
                all_dfs.push(df);
                println!("Ok for {}", url);
            },
            Err(e) => {
                eprintln!("Error on {}: {}", url, e);
                eprintln!("Error fetching market data: {}", e);
            },
        }
        thread::sleep(Duration::from_millis(1000));
    }
        
    if !all_dfs.is_empty() {
        let combined = all_dfs.into_iter().reduce(|acc, df| acc.vstack(&df).unwrap());
        if let Some(combined_df) = combined {
            output::write_table_out(&combined_df, "US_ELECTION_BETFAIR")?;

        } else {
            eprintln!("No dataframes to combine.");
        }
    }

    Ok(())
}
