mod commands;
mod comparison;
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

    //input::input_config();
    //gui::gui()?;
    json_system::comparison("./db/out");
    Ok(())
}
