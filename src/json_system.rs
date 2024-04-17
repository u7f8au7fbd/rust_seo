use crate::{cmd_color, commands};
use eframe::egui::debug_text::print;
use reqwest::header::AGE;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

pub fn search_file(dir_path: &str) -> Vec<String> {
    let entries = std::fs::read_dir(dir_path).expect("ディレクトリを読み込めませんでした。");

    let mut json_paths = Vec::new();

    for entry in entries {
        let file_path = entry.unwrap().path();
        let file_name = file_path.display();

        if file_path.is_file() && file_path.extension().unwrap() == "json" {
            json_paths.push(file_name.to_string());
        }
    }

    json_paths
}

pub fn get_data(fire_path: &str) -> Vec<String> {
    let mut file = File::open(fire_path).expect("ファイルを開けませんでした。");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("ファイルを読み込めませんでした。");

    let json: Value = serde_json::from_str(&contents).expect("JSONのパースに失敗しました。");

    let mut vec_string = Vec::new();

    if let Some(arr) = json.as_array() {
        for value in arr {
            if let Some(str_value) = value.as_str() {
                vec_string.push(str_value.to_string());
            }
        }
    }

    vec_string
}

pub fn comparison(file_path: &str) {
    let mut group: Vec<Vec<String>> = Vec::new();

    for i in search_file(file_path) {
        group.push(get_data(&i));
    }

    for i in 1..group.len() {
        commands::line();
        let before_vec = &group[i - 1];
        let after_vec = &group[i];

        for (index, (before, after)) in before_vec.iter().zip(after_vec.iter()).enumerate() {
            let after_index = after_vec.iter().position(|x| x == before);
            if before == after {
                println!(
                    "{}-{}:{}{}",
                    cmd_color!(green),
                    index + 1,
                    before,
                    cmd_color!(reset)
                );
            } else if after_vec.contains(before) {
                if index > after_index.unwrap() {
                    print!("{}↑", cmd_color!(green));
                } else {
                    print!("{}↓", cmd_color!(red));
                }
                println!(
                    "{}{}->{}:{}->{}{}",
                    cmd_color!(yellow),
                    index + 1,
                    after_index.unwrap() + 1,
                    before,
                    after,
                    cmd_color!(reset)
                );
            } else {
                println!("{}x{}:OUT{}", cmd_color!(red), index + 1, cmd_color!(reset));
            }
        }
    }
}
