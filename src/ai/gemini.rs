use crate::cls;
use ask_gemini::Gemini;
use inquire::{Confirm, Text};
use std::env::var;
use std::fs::{create_dir_all, File};
use std::io::{Error, Write};
use std::path::Path;

pub async fn ai() {
    let gemini = if let Ok(key) = var("API_KEY") {
        Gemini::new(Some(key.as_str()), None)
    } else {
        Gemini::new(
            Some(
                Text::new("Gemini api key :")
                    .prompt()
                    .expect("Missing value")
                    .as_str(),
            ),
            None,
        )
    };

    loop {
        let prompt = Text::new("").prompt().unwrap_or_default();
        match gemini.ask(prompt.as_str()).await {
            Ok(response) => {
                cls();
                let x = response.join("\n");
                if let Ok(home) = var("HOME") {
                    assert!(create_dir_all(format!("{home}/.gemini").as_str()).is_ok());
                    if Path::new(format!("{home}/.gemini").as_str()).is_dir() {
                        if let Ok(mut g) = File::options()
                            .write(true)
                            .create(true)
                            .append(true)
                            .open(format!("{home}/.gemini/gemini.log").as_str())
                        {
                            assert!(g
                                .write_all(format!("{prompt}\n\n{x}\n\n").as_bytes())
                                .is_ok());
                            assert!(g.sync_all().is_ok());
                            assert!(g.flush().is_ok());
                        }
                    }
                    print!("{x}");
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        }
        if Confirm::new("Quit ? :")
            .with_default(false)
            .prompt()
            .unwrap()
        {
            cls();
            break;
        }
    }
}

#[cfg(feature = "ai")]
pub async fn assistant(
    title: &str,
    description: &str,
    why: &str,
    benefits: &str,
    step: &str,
    expert: bool,
) -> Result<(), Error> {
    if expert {
        cls();
        return Ok(());
    }
    cls();
    println!("{title}\n\n{description}\n\n{why}\n\n{benefits}\n");
    if Confirm::new(format!("Do you want chat with the AI before configure the {step} ?").as_str())
        .with_default(true)
        .prompt()
        .unwrap()
    {
        ai().await;
    }
    Ok(())
}
