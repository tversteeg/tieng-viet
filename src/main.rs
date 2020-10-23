mod gui;

use anyhow::Result;
use std::io::{stdin, stdout, Write};
use termion::{
    clear::All,
    cursor::{Goto, Hide, Show},
    raw::IntoRawMode,
    screen::{ToAlternateScreen, ToMainScreen},
};

fn main() -> Result<()> {
    // Get the standard output stream and go to raw mode
    let mut stdout = stdout().into_raw_mode()?;

    // Write the initial message
    write!(
        stdout,
        "{}{}{}Welcome to Tiếng Việt, an interactive Vietnamese learning tool.{}\r\n",
        ToAlternateScreen,
        All,
        Goto(1, 1),
        Hide
    )?;

    // Make the output appear
    stdout.flush()?;

    // Initial selection menu
    let selected = gui::menu(&["Start", "Help", "Exit"])?;

    // Switch back to the main screen
    write!(stdout, "{}{}", ToMainScreen, Show)?;

    Ok(())
}