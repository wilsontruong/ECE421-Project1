use std::env;
use yahoo_finance_api as yahoo;
use std::io::{self, Write};

// Asynchronous function to check if a stock exists
async fn does_stock_exist(symbol: &str) -> bool {
    let provider = yahoo::YahooConnector::new();
    match provider.get_latest_quotes(symbol, "1d").await {
        Ok(_) => true,
        Err(_) => false,
    }
}

// fn fetch_stock_data() {

// }

// fn analyze_data() {

// }

#[tokio::main]
async fn main() {
    loop {
        // Print a prompt
        print!("Enter command: ");
        // Flush stdout to ensure the prompt is displayed
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Trim the newline character from the input
                let input = input.trim();

                // Match the input and perform actions
                match input {
                    "exit" => break, // Exit the loop and end the program
                    "--help" => println!("Help menu"), // Print the help menu
                    symbol => {
                        // Asynchronously check if the stock exists
                        let exists = does_stock_exist(symbol).await;
                        if exists {
                            println!("STOCK: {} exists", symbol);
                        } else {
                            println!("STOCK: {} does not exist", symbol);
                        }
                    }
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}