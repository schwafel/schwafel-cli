use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, BufRead};

#[derive(Serialize, Deserialize)]
struct SummarizeRequest {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct SummarizeResponse {
    summary_text: String,
}

async fn summarize(message: String, api_token: Option<String>) -> String {
    let body = SummarizeRequest { message };
    let authorization = match api_token {
        Some(token) => format!("Bearer {}", token),
        _ => "Bearer {API_TOKEN}".to_owned(),
    };
    let client = reqwest::Client::new();
    let res: SummarizeResponse = client
        .post("https://schwafel-worker.chriamue.net/summarize")
        .header("Authorization", authorization)
        .json(&body)
        .send()
        .await
        .unwrap()
        .json::<SummarizeResponse>()
        .await
        .unwrap();

    res.summary_text.to_string()
}

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let line = line.expect("Could not read line from standard in");
                let summary = summarize(line, None).await;
                println!("{}", summary);
            }
        }
        _ => {
            args.remove(0);
            let summary = summarize(args.join(" ").to_string(), None).await;
            println!("{}", summary);
        }
    }
}
