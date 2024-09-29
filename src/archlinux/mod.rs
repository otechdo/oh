use crate::ai::gemini::assistant;
use crate::archlinux::keyboard::configure_keyboard;
use crate::archlinux::locales::configure_locale;
use crate::archlinux::network::reflector::reflector;
use crate::archlinux::packages::install::packages;
use crate::archlinux::services::system::enable_services;
use crate::archlinux::time::configure_timezone;
use crate::os::Os;
use mirroring::configure_archlinux_mirrors;
use std::io::Error;

pub mod keyboard;
pub mod locales;
pub mod mirroring;
pub mod network;
pub mod packages;
pub mod services;
pub mod time;

pub async fn arch_install() -> Result<(), Error> {
    let mut app = Os::default();
    #[cfg(feature = "ai")]
    assert!(assistant(
        "Configure Mirrors.",
        "Configure the Pacman mirrors to select the fastest ones based on your location.",
        "This will make package installation and updates faster.",
        "Faster download speeds for packages\nReduced waiting time for installations and updates\nImproved overall system performance",
        "pacman mirrors").await.is_ok());
    assert!(configure_archlinux_mirrors(&mut app).is_ok());
    assert!(reflector(&mut app).await.is_ok());
    assert!(configure_keyboard(&mut app).await.is_ok());
    assert!(configure_timezone(&mut app).await.is_ok());
    assert!(configure_locale(&mut app).await.is_ok());
    assert!(packages().await);
    assert!(enable_services().await.is_ok());
    Ok(())
}
