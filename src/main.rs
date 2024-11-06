use clap::Parser;
use std::error::Error;

mod api;

#[derive(Parser, Debug)]
#[command(name = "Async HTTP Requester")]
#[command(about = "Fetches cryptocurrency prices asynchronously", long_about = None)]
struct Args {
    /// Cryptocurrency symbols (e.g., BTC ETH)
    #[arg(short, long, num_args = 1..)]
    symbols: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Args::parse();
    let mut tasks = Vec::new();

    for symbol in args.symbols {
        let symbol_upper = symbol.to_uppercase();
        tasks.push(tokio::spawn(async move {
            match api::fetch_price(&symbol_upper).await {
                Ok(price) => {
                    println!("The current price of {} is ${:.2}", symbol_upper, price);
                }
                Err(e) => {
                    eprintln!("Error fetching price data for {}: {}", symbol_upper, e);
                }
            }
        }));
    }

    for task in tasks {
        task.await?;
    }

    Ok(())
}
