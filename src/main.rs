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
    clean_console();
    set_utf8();
    input_config();
    get_google(
        "Rust言語".to_string(), // Corrected argument
        "AIzaSyAd5mMgR2D1OsftLC_pJqshqIWVkV5GQG0".to_string(),
        "90c2b21e39e984ff6".to_string(),
    )
    .await?;
    gui()?;
    Ok(())
}
