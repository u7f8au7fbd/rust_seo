// serdeとtomlクレートをインポートします
use serde::{Deserialize, Serialize};

// test.tomlに書き込む構造体を定義します
#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    age: u32,
    hobbies: Vec<String>,
}

fn main() {
    // tomlという名前のディレクトリを作成します
    std::fs::create_dir("toml").unwrap();

    // 構造体のインスタンスを作成します
    let config = Config {
        name: "Alice".to_string(),
        age: 25,
        hobbies: vec![
            "reading".to_string(),
            "coding".to_string(),
            "baking".to_string(),
        ],
    };

    // 構造体をTOML形式にシリアライズします
    let toml_str = toml::to_string(&config).unwrap();

    // tomlディレクトリの中にtest.tomlファイルに書き込みます
    std::fs::write("db/test.toml", toml_str).unwrap();

    // tomlディレクトリの中のtest.tomlファイルを読み込みます
    let toml_str = std::fs::read_to_string("db/test.toml").unwrap();

    // TOML形式の文字列を構造体にデシリアライズします
    let config: Config = toml::from_str(&toml_str).unwrap();

    // 構造体の内容を表示します
    println!("name: {}", config.name);
    println!("age: {}", config.age);
    println!("hobbies: {:?}", config.hobbies);
}
