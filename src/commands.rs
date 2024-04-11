use std::process::Command;
pub fn set_utf8() {
    Command::new("cmd")
        .args(["/C", "chcp 65001"])
        .output()
        .expect("UTF-8に設定できませんでした");
}
pub fn clean_console() {
    Command::new("cmd")
        .args(["/C", "cls"])
        .output()
        .expect("コンソールをリセットできませんでした");
}