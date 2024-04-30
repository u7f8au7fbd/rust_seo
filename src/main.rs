mod connect;
mod gui;
mod html;
mod input;
mod json;
#[macro_use]
mod macros;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cmd!(utf-8);
    cmd!(line);

    gui::gui()?;
    html::test_main();

    Ok(())
}
