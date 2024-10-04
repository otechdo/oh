#![allow(clippy::multiple_crate_versions)]
#[cfg(feature = "ai")]
pub mod ai;
#[cfg(feature = "archlinux")]
#[cfg(target_os = "linux")]
pub mod archlinux;
pub mod configuration;

#[cfg(feature = "gentoo")]
pub mod gentoo;
pub mod os;
pub mod utils;


#[cfg(feature = "ai")]
use crate::ai::gemini::ai;
#[cfg(feature = "archlinux")]
use crate::archlinux::arch_install;
#[cfg(feature = "ask")]
use crate::utils::{cls, confirm, select};
#[cfg(feature = "cli")]
use clap::{Arg, ArgMatches};
use std::io::{Error, ErrorKind};

#[cfg(feature = "gentoo")]
use crate::gentoo::install_gentoo;

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

#[cfg(feature = "ai")]
async fn chat(arg: &ArgMatches) -> Result<(), Error> {
    if let Some(chat) = arg.get_one::<bool>("chat") {
        if chat.eq(&true) {
            cls();
            #[cfg(feature = "ai")]
            ai().await;
            return Ok(());
        }
    }
    cls();
    Err(Error::new(ErrorKind::WouldBlock, "no ai re"))
}
#[tokio::main]
#[cfg(feature = "archlinux")]
async fn main() -> Result<(), Error> {
    cls();
    println!("Welcome to archlinux!");
    if confirm("Run installation ?").await.eq(&true) {
        return if chat(&oh()).await.is_err() {
            arch_install(confirm("Run in expert mode ? (no ai assistant)").await).await
        } else {
            cls();
            ai().await;
            arch_install(confirm("Run in expert mode ? (no ai assistant)").await).await
        };
    }
    cls();
    println!("Bye");
    Ok(())
}

#[tokio::main]
#[cfg(feature = "gentoo")]
async fn main() -> Result<(), Error> {
    cls();
    println!("Welcome to gentoo!");
    if confirm("Run installation ?").await.eq(&true) {
        return if chat(&oh()).await.is_err() {
            install_gentoo(confirm("Run installer in expert mode ? (no ai assistant) :").await).await
        } else {
            cls();
            ai().await;
            install_gentoo(confirm("Run in expert mode ? (no ai assistant)").await).await
        };
    }
    cls();
    println!("Bye");
    Ok(())
}