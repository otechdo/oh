#![allow(clippy::multiple_crate_versions)]

use std::env::var;
use std::io::Error;
use ask_gemini::Gemini;
use inquire::{Confirm, Text};

async fn cls()
{
    assert!(std::process::Command::new("clear").spawn().expect("linux").wait().is_ok());
}
async fn ai()
{
    let gemini = Gemini::new(Some(var("API_KEY").expect("no API_KEY founded").as_str()), None);
    loop {
        let prompt = Text::new("").prompt().unwrap_or_default();
        match gemini.ask(prompt.as_str()).await {
            Ok(response) => {
                cls().await;
                let x = response.join("\n");
                print!("{x}");
            }
            Err(e) => eprintln!("Error: {e}"),
        }
        if Confirm::new("Quit ? :").with_default(false).prompt().unwrap() {
            cls().await;
            break;
        }
    }
}

async fn confirm(prompt: &str) -> bool
{
    cls().await;
    if Confirm::new("Do you want chat with the AI before continue ?").with_default(true).prompt().unwrap() {
        ai().await;
        return Confirm::new(prompt).prompt().unwrap_or_default();
    }
    Confirm::new(prompt).with_default(false).prompt().unwrap_or_default()
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    cls().await;
    if confirm("Run installation ?").await.eq(&true) {
        cls().await;
        println!("Installation complete");
        return Ok(());
    }
    cls().await;
    println!("Bye");
    Ok(())
}
