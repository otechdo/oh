use crate::configuration::system::COUNTRIES;
use crate::{os::Os, select};
use std::io::Error;
use std::process::Command;
use crate::ai::gemini::assistant;

pub async fn configure_archlinux_mirrors(os: &mut Os) -> Result<(), Error> {
    #[cfg(feature = "ai")]
    assert!(assistant(
        "Configure Mirrors.",
        "Configure the Pacman mirrors to select the fastest ones based on your location.",
        "This will make package installation and updates faster.",
        "Faster download speeds for packages\nReduced waiting time for installations and updates\nImproved overall system performance",
        "pacman mirrors").await.is_ok());
    os.mirrors.country.clear();
    os.mirrors.protocol.clear();
    os.mirrors.save_at.clear();
    os.mirrors.sort.clear();
    os.mirrors
        .country
        .push_str(select("Enter your country:", COUNTRIES.map(String::from).to_vec()).as_str());
    os.mirrors.protocol.push_str(
        select(
            "Enter mirror protocol:",
            vec![
                "https".to_string(),
                "http".to_string(),
                "rsync".to_string(),
                "ftp".to_string(),
            ]
            .to_vec(),
        )
        .as_str(),
    );
    os.mirrors.sort.push_str(
        select(
            "Enter the sort option:",
            vec![
                "rate".to_string(),
                "delay".to_string(),
                "age".to_string(),
                "score".to_string(),
                "country".to_string(),
            ]
            .to_vec(),
        )
        .as_str(),
    );
    os.mirrors.save_at.push_str("/etc/pacman.d/mirrorlist");
    if os.mirrors.country.is_empty()
        || os.mirrors.protocol.is_empty()
        || os.mirrors.save_at.is_empty()
        || os.mirrors.sort.is_empty()
    {
        return configure_archlinux_mirrors(os);
    }
    if let Ok(mut c) = Command::new("pacman")
        .arg("-S")
        .arg("-y")
        .arg("-y")
        .arg("-u")
        .arg("--noconfirm")
        .current_dir("/tmp")
        .spawn()
    {
        assert!(c.wait().is_ok());
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}
