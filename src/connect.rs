use std::fs::File;
use std::io::Write;

pub async fn get_google(
    query: String,
    api_key: String,
    search_engine_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    for page in 0..10 {
        let client = reqwest::Client::new();
        let url =
            format!(
            "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&start={}&gl=jp&lr=lang_ja",
            api_key, search_engine_id, query, page*10
        );

        let response = client.get(&url).send().await?;
        let json = response.text().await?;
        let data: serde_json::Value = serde_json::from_str(&json)?;

        if let Some(links) = data["items"].as_array() {
            for link in links {
                if let Some(link_url) = link["link"].as_str() {
                    println!("{}", link_url);
                }
            }
        }
    }
    Ok(())
}
