use std::fs::read_to_string;
use std::process::Command;
use inquire::MultiSelect;
use crate::utils::confirm;

pub async fn packages() -> bool
{
    let mut install : Vec<String> = Vec::new();
    let mut packs:Vec<String> = Vec::new();
    let x = read_to_string("/tmp/packages").expect("Failed to read /tmp/packages");
    for line in x.lines() {
        packs.push(line.to_string());
    }
    loop {
        install.append(&mut MultiSelect::new("Select packages to install :", packs.to_vec()).prompt().expect("Failed to get packages"));
        if confirm("Do you want to continue ?").await.eq(&false) {
            break;
        }
    }
    Command::new("pacman").arg("-Syyu").args(install).spawn().expect("Failed to spawn package install").wait().expect("failed to wait on package install").success()
}