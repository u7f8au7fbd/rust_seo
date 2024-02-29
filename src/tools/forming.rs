use std::process::Command;

pub fn set_utf8() {
    Command::new("cmd")
        .args(["/C", "chcp 65001"])
        .output()
        .expect("UTF-8に設定できませんでした");
}

pub fn url_to_name(url: &str) -> String {
    url.replace("http://", "")
        .replace("https://", "")
        .replace("www.", "")
        .replace(['?', '!', '/'], "[]")
}

pub fn clean_temp() {
    if let Err(err) = std::fs::remove_dir_all("db/output") {
        eprintln!("outputをリセット出来ませんでした : {}", err);
    }
    std::fs::create_dir("db/output").unwrap_or_else(|err| {
        eprintln!("outputを再生成できませんでした : {}", err);
    });
}
