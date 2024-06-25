#![allow(clippy::multiple_crate_versions)]

use crate::arch::{Arch, Installer};
use argh::FromArgs;
use std::process::{exit, ExitCode};

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
fn main() -> ExitCode {
    let arch: Manager = argh::from_env();
    if arch.upgrade {
        exit(Arch::upgrade());
    }
    if arch.installer {
        exit(Arch::default().setup());
    }
    exit(1);
}
