mod commands;
use commands::*;
use gui::*;
mod gui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_utf8();
    gui()?;
    Ok(())
}
