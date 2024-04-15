mod commands;
use gui::*;
mod gui;
mod input;
use input::*;
mod comparison;
mod connect;
mod test;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    input_config();
    gui()?;
    */
    commands::clear();
    commands::utf8();

    test::vec_test();

    Ok(())
}
