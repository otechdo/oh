#![allow(clippy::multiple_crate_versions)]

use crate::arch::{Arch, Installer};
use argh::FromArgs;

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
    #[argh(option, short = 'i', default = "true")]
    /// run the installer
    installer: bool,
}

fn main() {
    let arch: Manager = argh::from_env();

    if arch.installer {
        assert_eq!(Arch::default().setup(), 0);
    }
}
