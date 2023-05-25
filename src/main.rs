extern crate reqwest;

extern crate tokio;
use std::process;

use minigoogle::{get_user_query, make_api_request, display_label, display_summary, display_results};
use tokio::main;

mod config;
use config::Config;

#[main]
async fn main() {
    let config = Config::new();

    let query = get_user_query().unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });

    let data = make_api_request(query, config.api_key).await.unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });

    display_summary(data.get("knowledge_panel"));

    display_label("Search results");

    display_results(&data);
}