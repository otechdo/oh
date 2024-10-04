use std::io::Error;
use std::process::Command;
use crate::os::Os;

#[cfg(feature = "ask")]
use inquire::{Confirm, Select, Text};

pub fn cls() {
    assert!(Command::new("clear").spawn().expect("linux").wait().is_ok());
}

#[cfg(feature = "ask")]
pub fn select(prompt: &str, opts: Vec<String>) -> String {
    cls();
    Select::new(prompt, opts.to_vec())
        .prompt()
        .unwrap_or_default()
}
#[cfg(feature = "ask")]
pub fn text(prompt: &str) -> String {
    Text::new(prompt).prompt().unwrap_or_default()
}
pub async fn ask(app: &mut Os, c: fn(&mut Os) -> Result<(), Error>) -> Result<(), Error> {
    cls();
    assert!(c(app).is_ok());
    Ok(())
}

#[cfg(feature = "ask")]
pub async fn confirm(prompt: &str) -> bool {
    Confirm::new(prompt)
        .with_default(false)
        .prompt()
        .expect("Failed to prompt")
}
