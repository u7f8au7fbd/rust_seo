mod commands;
use gui::*;
mod gui;
mod input;
use input::*;
mod comparison;
mod connect;
mod test;

#[macro_use]
mod macros;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    commands::clear();
    commands::utf8();

    //input_config();
    //gui()?;
    test::test_main();
    Ok(())
}
