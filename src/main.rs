mod commands;
mod connect;
mod gui;
mod input;
mod json_system;
#[macro_use]
mod macros;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    commands::clear();
    commands::utf8();

    input::config();
    gui::gui()?;
    Ok(())
}
