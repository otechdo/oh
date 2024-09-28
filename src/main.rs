#![allow(clippy::multiple_crate_versions)]
#[cfg(feature = "ai")]
pub mod ai;
pub mod archlinux;
pub mod configuration;
pub mod gentoo;
pub mod os;
pub mod utils;

use crate::ai::gemini::ai;
use crate::archlinux::arch_install;
use crate::gentoo::install_gentoo;
use crate::utils::{cls, confirm, select};
use clap::{Arg, ArgMatches};
use std::io::Error;

pub fn oh() -> ArgMatches {
    clap::Command::new("oh")
        .arg(
            Arg::new("chat")
                .help("chat with ai")
                .num_args(0)
                .long("chat"),
        )
        .get_matches()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = oh();
    if let Some(chat) = app.get_one::<bool>("chat") {
        if chat.eq(&true) {
            cls();
            #[cfg(feature = "ai")]
            ai().await;
            return Ok(());
        }
    }
    cls();
    if confirm("Run installation ?").await.eq(&true) {
        let os = select(
            "What distribution you want to install ?",
            vec!["archlinux".to_string(), "gentoo".to_string()],
        );
        if confirm(format!("run {os} installer ?").as_str()).await {
            match os.as_str() {
                "archlinux" => assert!(arch_install().await.is_ok()),
                "gentoo" => assert!(install_gentoo().await.is_ok()),
                _ => unreachable!("no possible"),
            };
        }
        println!("{os} is installed successfully");
        return Ok(());
    }
    cls();
    println!("Bye");
    Ok(())
}
