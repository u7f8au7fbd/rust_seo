use select::document::Document;
use select::predicate::Name;

use url::form_urlencoded;

// クエリ文字列をエンコードする関数
pub fn encode_query(query: &str) -> String {
    form_urlencoded::Serializer::new(String::new())
        .append_pair("q", query)
        .finish()
}

pub fn get_google_search_url(query: &str) -> String {
    let encoded_query = encode_query(query);
    format!("https://www.google.com/search?{}", encoded_query)
}

// Google検索のURLにアクセスして、検索結果のURLを取得する関数
pub async fn get_search_results(url: &str) -> Result<Vec<String>, reqwest::Error> {
    // HTTP GETリクエストを送信して、レスポンスのHTMLを取得する
    let html = reqwest::get(url).await?.text().await?;

    // HTMLを解析して、Documentオブジェクトを作るx
    let document = Document::from(html.as_str());

    // Documentオブジェクトから、<a>タグのhref属性を抽出する
    let mut results = Vec::new();
    for node in document.find(Name("a")) {
        if let Some(href) = node.attr("href") {
            // href属性が/url?q=で始まっているものだけを取り出す
            if href.starts_with("/url?q=") {
                // /url?q=を取り除いて、URLをデコードする
                let result = href.trim_start_matches("/url?q=").to_string();
                let result = form_urlencoded::parse(result.as_bytes())
                    .map(|(key, _)| key)
                    .collect::<Vec<_>>()
                    .join("");
                results.push(result);
            }
        }
    }

    // 検索結果のURLを返す
    Ok(results)
}

