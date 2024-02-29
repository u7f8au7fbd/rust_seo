use reqwest::Error;
mod tools;
use tools::connect;
use tools::forming;

#[tokio::main]
async fn main() -> Result<(), Error> {
    forming::clean_temp();
    connect::get("https://www.rust-lang.org").await?;
    connect::get("https://www.google.com").await?;
    connect::get("https://www.yahoo.co.jp").await?;
    Ok(())
}
