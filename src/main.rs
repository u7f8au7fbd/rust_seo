use reqwest::Error;
mod gui;
mod tools;
use tools::connect;
use tools::forming;
use tools::search;
use tools::serialization;

#[tokio::main]
async fn main() -> Result<(), Error> {
    gui::gui();

    forming::set_utf8(); //UTF-8に設定
    forming::clean_temp(); //DBファイルをリセット
    serialization::input_output(); //DBファイルに書き込み
                                   //以下はhtmlを取得実験
    connect::get("https://www.google.com").await?;
    connect::get("https://www.bing.com/").await?;
    connect::get("https://www.yahoo.co.jp").await?;
    println!("インプット終了");
    // 検索したいクエリを定義する
    let query = "SEO対策";

    // Google検索のURLを取得する
    let url = search::get_google_search_url(query);
    let results = search::get_search_results(&url).await.unwrap();
    for result in results.iter().take(18) {
        println!("{}", result);
    }
    Ok(())
}
