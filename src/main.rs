#![allow(clippy::multiple_crate_versions)]

use std::collections::HashMap;
use std::process::{exit, ExitCode};
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
    #[argh(switch, short = 'i', description = "start the installer")]
    installer: bool,
    #[argh(switch, short = 'u', description = "upgrade the system")]
    upgrade: bool,
}

fn run(arch: HashMap<&dyn Fn() -> i32, bool>) -> i32
{
    for (c, b) in arch {
        if b {
            assert_eq!(c(), 0);
        }
    }
    0
}
fn setup() -> i32
{
    Arch::default().setup()
}
fn upgrade() -> i32
{
    Arch::upgrade()
}

fn main() -> ExitCode {
    let arch: Manager = argh::from_env();
    let mut all: HashMap<&dyn Fn() -> i32, bool> = HashMap::new();
    all.insert(&setup, arch.installer);
    all.insert(&upgrade, arch.upgrade);
    exit(run(all));
}
