pub fn url_to_name(url: &str) -> String {
    url.replace("http://", "")
        .replace("https://", "")
        .replace("www.", "")
        .replace(['?', '!', '/'], "-")
}

pub fn clean_temp() {
    let _ = std::fs::remove_dir_all("temp");
    let _ = std::fs::create_dir("temp");
}
