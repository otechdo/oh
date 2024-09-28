use crate::os::Os;
use inquire::{Confirm, Select, Text};
use std::io::Error;
use std::process::Command;

pub fn cls() {
    assert!(Command::new("clear").spawn().expect("linux").wait().is_ok());
}

pub fn select(prompt: &str, opts: Vec<String>) -> String {
    cls();
    Select::new(prompt, opts.to_vec())
        .prompt()
        .unwrap_or_default()
}
pub fn text(prompt: &str) -> String {
    Text::new(prompt).prompt().unwrap_or_default()
}
pub async fn ask(app: &mut Os, c: fn(&mut Os) -> Result<(), Error>) -> Result<(), Error> {
    cls();
    assert!(c(app).is_ok());
    Ok(())
}

pub async fn confirm(prompt: &str) -> bool {
    Confirm::new(prompt)
        .with_default(false)
        .prompt()
        .expect("Failed to prompt")
}
