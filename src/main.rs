use schwafel_worker::summarize;
use std::env;
use std::io::{self, BufRead};

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    let api_token = match env::var("API_TOKEN") {
        Ok(api_token) => Some(api_token),
        _ => None
    };

    match args.len() {
        1 => {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let line = line.expect("Could not read line from standard in");
                let summary = summarize(line, api_token.clone()).await;
                println!("{}", summary);
            }
        }
        _ => {
            args.remove(0);
            let summary = summarize(args.join(" ").to_string(), api_token).await;
            println!("{}", summary);
        }
    }
}
