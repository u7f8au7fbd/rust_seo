use reqwest::Error;
mod tools;
use tools::connect;
use tools::forming;
use tools::search;
use tools::serialization;

#[tokio::main]
async fn main() -> Result<(), Error> {
    forming::set_utf8();
    forming::clean_temp();
    connect::get("https://www.google.com").await?;
    connect::get("https://www.bing.com/").await?;
    connect::get("https://www.yahoo.co.jp").await?;
    println!("インプット終了");
    // 検索したいクエリを定義する
    let query = "おいしい店";

    // Google検索のURLを取得する
    let url = search::get_google_search_url(query);

    // Google検索のURLにアクセスして、検索結果のURLを取得する
    let results = search::get_search_results(&url).await.unwrap();

    // 検索結果のURLを上から順番に30つまで出力する
    for result in results.iter().take(18) {
        println!("{}", result);
    }
    Ok(())
}
