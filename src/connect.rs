use crate::forming;
use reqwest::Error;
use std::fs::File;
use std::io::Write;
pub async fn get(url: &str) -> Result<(), Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let mut file =
        File::create(format!("temp/{}.html", forming::url_to_name(url))).expect("Error-1");
    file.write_all(body.as_bytes()).expect("Error-2");
    println!("Downloaded: {}", url);
    Ok(())
}
