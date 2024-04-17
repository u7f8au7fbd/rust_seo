mod commands;
mod comparison;
mod connect;
mod gui;
mod input;
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
