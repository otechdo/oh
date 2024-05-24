#![allow(clippy::multiple_crate_versions)]

use crate::arch::{Arch, Installer};
use std::env::args;
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
    let args: Vec<String> = args().collect();
    if args.len() == 2 && args.get(1).unwrap().eq("setup") {
        exit(Arch::default().setup());
    }
    println!("{} setup", args[0]);
    exit(1);
}
