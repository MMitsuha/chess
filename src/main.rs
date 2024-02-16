use std::error::Error;

use log::{debug, error, info};

use console::{style, Term};

struct Game {}

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    term.clear_screen()?;
    term.write_line("Welcome to Mitsuha's chess game!\n")?;

    Ok(())
}
