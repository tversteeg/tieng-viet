// Use the new split_once function for strings
#![feature(str_split_once)]

mod grammar;
mod gui;

use anyhow::Result;
use grammar::{sentence::Sentence, Generate};
use std::io::{self, Write};
use termion::{
    clear::All,
    cursor::{Goto, Hide, Show},
    raw::IntoRawMode,
    screen::{ToAlternateScreen, ToMainScreen},
};

//fn program(stdout: &mut Stdout) -> Result<()> {
fn main() -> Result<()> {
    // Re-open stdout with raw mode to close it again
    let stdout = io::stdout();
    let mut stdout = stdout
        .lock()
        .into_raw_mode()
        .expect("Could not get raw mode in terminal for stdout");

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

    // Write some random sentences.
    let mut rng = rand::thread_rng();
    for _ in 1..10 {
        let words = Sentence::generate(&mut rng)?;
        words.into_iter().for_each(|word| {
            write!(stdout, "{} ", word.to_string()).expect("Could not write to output")
        });
        write!(stdout, "\r\n")?;
    }
    stdout.flush()?;

    // Initial selection menu
    let _selected = gui::menu(&["Start", "Help", "Exit"])?;

    // Reset the terminal
    write!(stdout, "{}{}", ToMainScreen, Show)?;
    stdout.flush()?;

    Ok(())
}
