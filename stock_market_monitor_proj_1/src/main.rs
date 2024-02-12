use std::env;
use yahoo_finance as yahoo;

// fn fetch_stock_data() {

// }

// fn analyze_data() {

// }

use std::io::{self, Write};

fn main() {
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
                    _ => println!("STOCK: {}", input), // Echo the input prefixed with "STOCK: "
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}