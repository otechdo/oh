#![allow(clippy::multiple_crate_versions)]

use crate::arch::{Arch, Installer};
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

fn main() -> ExitCode {
    exit(Arch::default().setup())
}
