use reqwest::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;

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

#[tokio::main]
async fn main() {
    // TODO: 日付を注入できるようにする
    let response = reqwest::get(
        "https://opendata.corona.go.jp/api/Covid19JapanAll?date=20220123&dataName=東京都",
    )
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    let ret: Response = serde_json::from_str(&response).unwrap();

    println!("{}", ret.itemList.first().unwrap().npatients);
}
