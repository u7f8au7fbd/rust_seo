mod commands;
use commands::*;
use gui::*;
mod gui;
mod input;
use input::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_utf8();
    input_config();
    gui()?;
    Ok(())
}
