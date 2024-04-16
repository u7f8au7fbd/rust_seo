use std::process::Command;
pub fn utf8() {
    Command::new("cmd")
        .args(["/C", "chcp 65001"])
        .output()
        .expect("UTF-8に設定できませんでした");
}
pub fn clear() {
    Command::new("cmd")
        .args(["/C", "cls"])
        .output()
        .expect("コンソールをリセットできませんでした");
    println!("------------------------")
}

pub fn line() {
    println!("------------------------")
}

pub fn format_path(path: &str) -> String {
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    let formatted_path = path
        .chars()
        .map(|c| if invalid_chars.contains(&c) { ' ' } else { c })
        .collect();
    formatted_path
}
