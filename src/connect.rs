use chrono::Local;
use std::fs::File;
use std::io::Write;

pub async fn get_google(
    query: String,
    api_key: String,
    search_engine_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut resoult: Vec<String> = Vec::new();
    let mut count = 0;
    for page in 0..10 {
        let client = reqwest::Client::new();
        let url = format!(
            "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&start={}&gl=jp&lr=lang_ja",
            api_key, search_engine_id, query, 10*page
        );

        let response = client.get(&url).send().await?;
        let json = response.text().await?;
        let data: serde_json::Value = serde_json::from_str(&json)?;

        if let Some(links) = data["items"].as_array() {
            for link in links {
                if let Some(link_url) = link["link"].as_str() {
                    count += 1;
                    println!("{}:{}", count, link_url);
                    resoult.push(link_url.to_string());
                }
            }
        }
    }

    let json_data = serde_json::to_string(&resoult)?;
    let file_path = format!("./output/URLs({})[{}].json", query, get_now());
    let mut file = File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

fn get_now() -> String {
    let dt = Local::now();
    dt.format("%Y年%m月%d曜日_%H時%M分%S秒").to_string()
}
