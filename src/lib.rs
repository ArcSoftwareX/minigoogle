use serde_json::Value;

extern crate colored;
use colored::Colorize;

use std::{io::{self, Write}, error::Error, process};

fn display_result_card(value: &Value, idx: usize) {
    println!("{} {}", format!("{}.", (idx + 1)).bright_blue(), value.get("title").expect("Failed to get title field").as_str().expect("Failed to cast title to String").truecolor(132, 204, 22).bold());
    println!("{}", value.get("url").expect("Failed to get url field").as_str().expect("Failed to cast url to String").truecolor(234, 179, 8).bold());
    println!("{}\n", value.get("description").expect("Failed to get description").as_str().expect("Failed to cast description to String").bright_white());
}

pub fn display_results(value: &Value) {
    for (i, val) in value.get("results").expect("Failed to get results field").as_array().expect("Failed to cast results to Array").iter().enumerate() {
        display_result_card(val, i);
    }
}

pub fn display_summary(value: Option<&Value>) {
    if let Some(value) = value {
        if !value.is_null() {
            println!("\n{} {}\n", "Summary for".truecolor(100, 100, 100).bold(), value.as_object().expect("Failed to cast panel to Object").get("name").expect("Failed to get panel name").as_str().expect("Failed to cast name to String").truecolor(132, 204, 22).bold());
            println!("{}", value.as_object().expect("Failed to cast panel to Object").get("description").expect("Failed to get panel description").as_object().expect("Failed to cast description to Object").get("text").expect("Failed to get description text").as_str().expect("Failed to cast description text to String").bright_white());
        }
    }
}

pub fn display_label(text: &str) {
    println!("\n{}\n", text.truecolor(100, 100, 100).bold());
}

pub fn get_user_query() -> Result<String, Box<dyn Error>> {
    print!("\n{} > ", "minigoogle search".on_bright_white().black().bold());
    io::stdout().flush()?;

    let mut input_buf = String::new();
    io::stdin().read_line(&mut input_buf)?;

    Ok(input_buf)
}

pub async fn make_api_request(query: String, api_key: &str) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.get("https://google-search74.p.rapidapi.com")
        .header("X-Rapidapi-Key", api_key)
        .header("X-Rapidapi-Host", "google-search74.p.rapidapi.com")
        .query(&[("query", query)])
        .send()
        .await?;

    if !response.status().is_success() {
        println!("Failed to get data from API");
        process::exit(1);
    }
    
    Ok(serde_json::from_str(&response.text().await?)
        .unwrap_or_else(|err| {
            println!("Failed to deserialize result: {}", err);
            process::exit(1);
        }))
}