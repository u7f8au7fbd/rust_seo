mod commands;
mod connect;
mod gui;
mod html;
mod input;
mod json;
#[macro_use]
mod macros;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    commands::clear();
    commands::utf8();

    gui::gui()?;

    Ok(())
}
