use serde::Deserialize;
use std::fs::File;
use std::io::Read;
pub fn input_config() {
    // Open the config file
    let mut file = File::open("./input/config.json").expect("ファイルが見つかりません");

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("ファイルの読み込みに失敗");

    // Deserialize the JSON contents into a Config struct
    let config: Config =
        serde_json::from_str(&contents).expect("コンフィグファイルの読み込みに失敗");

    println!("{}", config.api_key);
    println!("{}", config.search_engine_id);
    println!("{}", config.search_words);
}

#[derive(Debug, Deserialize)]
struct Config {
    api_key: String,
    search_engine_id: String,
    search_words: String,
}