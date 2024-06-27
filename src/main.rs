#![allow(clippy::multiple_crate_versions)]

use std::io::BufRead;
use std::process::{exit, Command, ExitCode};

use crate::arch::{Arch, Installer};
use argh::FromArgs;
use inquire::MultiSelect;

mod arch;
mod base;
mod boot;
mod conf;
mod desktop;
mod diy;
mod hack;
mod programming;
mod server;
mod window;

#[derive(FromArgs)]
/// install and manage an archlinux
struct Manager {
    #[argh(switch, short = 's', description = "start the installer")]
    setup: Option<bool>,

    #[argh(switch, short = 'u', description = "upgrade the system")]
    upgrade: Option<bool>,

    #[argh(switch, short = 'c', description = "upgrade the cache system")]
    cache: Option<bool>,

    #[argh(switch, short = 'i', description = "install packages")]
    install: Option<bool>,

    #[argh(switch, short = 'r', description = "uninstall packages")]
    remove: Option<bool>,
}

fn uuid() -> String {
    let output = Command::new("blkid").output().expect("").stdout;
    let mut lines = output.lines();

    let uuid = lines
        .find(|line| {
            let line = line.as_ref().unwrap();
            line.contains("UUID=") && !line.contains("TYPE=\"vfat\"")
        })
        .ok_or("UUID not found or partition is VFAT")
        .unwrap()
        .unwrap();
    let art: Vec<&str> = uuid.split("UUID=\"").collect();
    art.last().unwrap().replace('"', "")
}
fn main() -> ExitCode {
    let arch: Manager = argh::from_env();
    if arch.upgrade.is_some() {
        exit(Arch::upgrade());
    } else if arch.cache.is_some() {
        exit(Arch::cache());
    } else if arch.install.is_some() {
        let x = MultiSelect::new("Select packages to install : ", Arch::packages())
            .prompt()
            .unwrap();
        exit(Arch::i(&x));
    } else if arch.remove.is_some() {
        let x = MultiSelect::new("Select packages to install : ", Arch::packages())
            .prompt()
            .unwrap();
        exit(Arch::remove(&x));
    }
    if arch.setup.is_some() {
        exit(Arch::default().setup(uuid()));
    }
    exit(1);
}
