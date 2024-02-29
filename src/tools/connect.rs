use crate::tools::forming;
use reqwest::Error;
use std::fs::File;
use std::io::Write;

pub async fn get(url: &str) -> Result<(), Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let mut file = File::create(format!("db/output/{}.html", forming::url_to_name(url)))
        .expect("接続できませんでした");
    file.write_all(body.as_bytes())
        .expect("ダウンロードできませんでした");
    println!("接続: {}", url);
    Ok(())
}
