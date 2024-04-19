use std::fs;

pub fn test() {}

pub async fn get_html(urls: Vec<String>) -> reqwest::Result<()> {
    for url in urls {
        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        let file_name = format!("./db/out/html/{}.html", url);
        fs::write(file_name, body);
    }
    Ok(())
}
