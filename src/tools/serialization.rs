use serde::{Deserialize, Serialize};

// 取得したURLのリストを保持する構造体のアトリビュート
#[derive(Serialize, Deserialize)]
pub struct LinkList {
    url: Vec<String>,
}

pub fn input_output() {
    // 構造体のインスタンスを作成
    let link_list = LinkList {
        url: vec!["".to_string()],
    };

    let toml_str = toml::to_string(&link_list).unwrap();

    std::fs::write("db/input/url.toml", toml_str).unwrap();

    let toml_str = std::fs::read_to_string("db/input/url.toml").unwrap();

    let config: LinkList = toml::from_str(&toml_str).unwrap();

    println!("URL: {:?}", config.url);
}
