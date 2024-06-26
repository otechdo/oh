#![allow(clippy::multiple_crate_versions)]

use std::io::BufRead;
use std::process::{exit, Command, ExitCode};

use argh::FromArgs;

use crate::arch::{Arch, Installer};

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
    #[argh(switch, short = 'i', description = "start the installer")]
    installer: bool,
    #[argh(switch, short = 'u', description = "upgrade the system")]
    upgrade: bool,
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
    println!("{}", uuid());
    let arch: Manager = argh::from_env();
    if arch.upgrade {
        exit(Arch::upgrade());
    }
    if arch.installer {
        exit(Arch::default().setup(uuid()));
    }
    exit(1);
}
