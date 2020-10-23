use anyhow::{anyhow, Result};
use std::io::{stdin, stdout, Write};
use termion::{
    color::{Cyan, Fg, Reset},
    cursor::{Restore, Save},
    event::Key,
    input::TermRead,
};

/// Draw a menu from an array of options, and return the index selected.
pub fn menu(options: &[&str]) -> Result<usize> {
    let mut selected = 0;

    draw_menu(options, selected)?;

    // Handle keyboard events.
    for c in stdin().keys() {
        match c.expect("Could not get key from stdin for menu") {
            Key::Up => {
                if selected == 0 {
                    // Wrap the cursor around
                    selected = options.len();
                }
                selected -= 1;
            }
            Key::Down => {
                selected += 1;
                if selected == options.len() {
                    // Wrap the cursor around
                    selected = 0;
                }
            }
            Key::Char('\n') => {
                return Ok(selected);
            }
            _ => (),
        }

        draw_menu(options, selected)?;
    }

    Err(anyhow!("Stdin key capture returned prematurely"))
}

fn draw_menu(options: &[&str], selected: usize) -> Result<()> {
    let mut stdout = stdout();

    // Put all options on their own line
    let menu = options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            format!(
                "{}{} {}.{} {}\r\n",
                Fg(Cyan),
                // Draw a symbol for the item that will be selected
                if index == selected { ">" } else { " " },
                // The key that can be pressed to select this option
                index + 1,
                Fg(Reset),
                option
            )
        })
        .collect::<String>();

    write!(stdout, "{}{}{}", Save, menu, Restore,);

    // Show the output on the screen
    stdout.flush()?;

    Ok(())
}
