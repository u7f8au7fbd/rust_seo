use reqwest::Error;
mod gui;
mod seo;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Error> {
    gui::gui().unwrap();
    Ok(())
}
