mod commands;
use commands::*;
use gui::*;
mod gui;
mod input;
use input::*;
mod connect;
use connect::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_utf8();
    input_config();
    get_google(
        "Rust言語".to_string(), // Corrected argument
        "AIzaSyBFa1HM33_pgrfzBXjL04QHKqyoebcPnkg".to_string(),
        "e30a673cf74764c2d".to_string(),
    )
    .await?;
    gui()?;
    Ok(())
}
