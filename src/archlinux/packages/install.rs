use std::fs::read_to_string;
use std::process::Command;

#[cfg(feature = "ask")]
use crate::utils::confirm;
#[cfg(feature = "ask")]
use inquire::MultiSelect;
#[cfg(feature = "ai")]
use crate::ai::gemini::ai;

#[cfg(feature = "archlinux")]
pub async fn packages(expert: bool) -> bool {
    let mut install: Vec<String> = Vec::new();
    let mut packs: Vec<String> = Vec::new();
    let x = read_to_string("/tmp/packages").expect("Failed to read /tmp/packages");
    for line in x.lines() {
        packs.push(line.to_string());
    }
    'install: loop {
        #[cfg(feature = "ai")]
        if expert.eq(&false) {
            ai().await;
        }
        'add: loop {
            install.append(
                &mut MultiSelect::new("Select packages to install :", packs.to_vec())
                    .prompt()
                    .expect("Failed to get packages"),
            );
            if confirm("Do you want to continue ?").await.eq(&false) {
                break 'add;
            }
        }
        if install.is_empty().eq(&false)
            && Command::new("pacman")
            .arg("-Syu")
            .args(install.to_vec())
            .spawn()
            .expect("Failed to spawn package install")
            .wait()
            .expect("failed to wait on package install")
            .success()
        {
            install.clear();
            if confirm("Do you want to continue ?").await.eq(&false) {
                break 'install;
            }
        }
    }
    true
}
