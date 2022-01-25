use reqwest::*;

#[tokio::main]
async fn main() -> Result<()> {
    let response = reqwest::get(
        "https://opendata.corona.go.jp/api/Covid19JapanAll?date=20220123&dataName=東京都",
    )
    .await?;

    let ret = response.text().await?;

    println!("{}", ret);

    Ok(())
}
