// Use the new split_once function for strings
#![feature(str_split_once)]

mod grammar;
mod gui;

use anyhow::Result;
use grammar::{sentence::Sentence, Generate};
use std::io::{stdout, Write};
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

    // Write a random sentence.
    let mut rng = rand::thread_rng();
    let words = Sentence::generate(&mut rng)?;
    words.into_iter().for_each(|word| {
        write!(stdout, "{} ", word.to_string());
    });
    write!(stdout, "\r\n");

    // Initial selection menu
    let _selected = gui::menu(&["Start", "Help", "Exit"])?;

    // Switch back to the main screen
    write!(stdout, "{}{}", ToMainScreen, Show)?;

    Ok(())
}
