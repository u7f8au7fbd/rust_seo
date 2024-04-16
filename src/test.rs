use std::fs::File;
use std::io::Read;
#[macro_use]
mod macros;
use crate::cmd_color;

pub fn read_json(file_path: &str) {
    let mut file = File::open(file_path).expect("ファイルを開けませんでした。");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("ファイルが読み込めませんでした。");

    let json: serde_json::Value =
        serde_json::from_str(&contents).expect("Json形式に変換できませんでした。");

    println!("{:#?}", json);
}

pub fn search_fire(dir_path: &str) -> Vec<String> {
    let entries = std::fs::read_dir(dir_path).expect("ディレクトリを読み込めませんでした。");

    let mut json_paths = Vec::new(); // Create an empty vector to store the JSON paths

    for entry in entries {
        let file_path = entry.unwrap().path();
        let file_name = file_path.display();

        if file_path.is_file() && file_path.extension().unwrap() == "json" {
            println!("{}", file_name);
            json_paths.push(file_name.to_string()); // Add the JSON path to the vector
        }
    }

    json_paths
}

pub fn print_json(json_paths: Vec<String>) {
    for json_path in json_paths {
        read_json(&json_path);
    }
}

pub fn compare(json_paths: Vec<String>) {
    for json_path in json_paths {
        //jsonを読み込む
        let mut file = File::open(json_path).expect("ファイルを開けませんでした。");
        //1つ前のjsonと比較する
    }
}

pub fn vec_test() {
    let test1: Vec<String> = vec![
        "あ".to_string(),
        "い".to_string(),
        "う".to_string(),
        "え".to_string(),
        "お".to_string(),
    ];

    let test2: Vec<String> = vec![
        "あ".to_string(),
        "い".to_string(),
        "ん".to_string(),
        "お".to_string(),
        "え".to_string(),
    ];

    let group = [test1, test2];

    for i in 1..group.len() {
        let before_vec = &group[i - 1];
        let after_vec = &group[i];

        for (index, (before, after)) in before_vec.iter().zip(after_vec.iter()).enumerate() {
            if before == after {
                println!(
                    "{}{}:{}{}",
                    cmd_color!(green),
                    index + 1,
                    before,
                    cmd_color!(reset)
                );
            } else if after_vec.contains(before) {
                println!(
                    "{}{}:{}{}{}",
                    cmd_color!(yellow),
                    index + 1,
                    before,
                    after,
                    cmd_color!(reset)
                );
            } else {
                println!("{}{}:OUT{}", cmd_color!(red), index + 1, cmd_color!(reset));
            }
        }
    }
}
