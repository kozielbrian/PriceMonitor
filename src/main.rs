use crate::alert::{Alert, AlertData, LogLevels};
use crate::gemini::get_startup_prices;
use anyhow::Result;
use chrono::prelude::*;
use clap::Parser;
use std::thread;
use std::time::Duration;

mod alert;
mod gemini;
mod prices;

const NUM_HOURS: usize = 24;

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// The currency trading pair, or ALL
    #[arg(short, long)]
    pub currency: String,

    /// standard deviation threshold. eg. 1
    #[arg(short, long, default_value_t = 1f64)]
    pub deviation: f64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let trading_pair = args.currency;

    let mut current_prices = get_startup_prices(&trading_pair).await.unwrap();

    println!("Startup prices: {:?}", current_prices);

    loop {
        // Sleep for an hour and then check up on new prices
        thread::sleep(Duration::from_secs(3600));
        let new_prices = get_startup_prices(&trading_pair).await.unwrap();

        // Compare the new incoming change in price to the standard deviation
        let abs_diff = (new_prices.values[0] - current_prices.mean).abs();
        if abs_diff > (args.deviation * current_prices.stdev) {
            let alert = Alert {
                timestamp: Local::now().to_rfc3339().to_string(),
                level: LogLevels::Info,
                trading_pair: trading_pair.clone(),
                deviation: true,
                data: AlertData {
                    last_price: current_prices.values[0].to_string(),
                    average: current_prices.mean.to_string(),
                    change: abs_diff.to_string(),
                    sdev: current_prices.stdev.to_string(),
                },
            };
            println!("{}", serde_json::to_string(&alert).unwrap());
        }

        current_prices = new_prices;
    }
}
