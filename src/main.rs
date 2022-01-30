use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct ErrorInfo {
    errorFlag: String,
    errorCode: Option<String>,
    errorMessage: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Item {
    date: String,
    name_jp: String,
    npatients: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    errorInfo: ErrorInfo,
    itemList: Vec<Item>,
}

#[derive(Parser)]
struct Cli {
    date: String,
    prefecture: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let response = reqwest::get(format!(
        "https://opendata.corona.go.jp/api/Covid19JapanAll?date={}&dataName={}",
        args.date, args.prefecture
    ))
    .await?
    .text()
    .await?;

    let ret: Response = serde_json::from_str(&response)?;

    println!("{}", ret.itemList.first().unwrap().npatients);

    Ok(())
}
