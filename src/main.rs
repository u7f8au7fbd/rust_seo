mod commands;
use commands::*;
use gui::*;
mod gui;
mod input;
use input::*;
mod comparison;
mod connect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    clean_console();
    set_utf8();
    input_config();
    gui()?;
    Ok(())
}
