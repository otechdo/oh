use crate::archlinux::keyboard::configure_keyboard;
use crate::archlinux::locales::configure_locale;
use crate::archlinux::network::reflector::reflector;
use crate::archlinux::packages::install::packages;
use crate::archlinux::time::configure_timezone;
use crate::os::Os;
use mirroring::configure_archlinux_mirrors;
use std::io::Error;
use crate::archlinux::services::system::enable_services;

pub mod keyboard;
pub mod locales;
pub mod services;
pub mod mirroring;
pub mod network;
pub mod packages;
pub mod time;

pub async fn arch_install() -> Result<(), Error> {
    let mut app = Os::default();
    assert!(configure_archlinux_mirrors(&mut app).await.is_ok());
    assert!(reflector(&mut app).await.is_ok());
    assert!(configure_keyboard(&mut app).await.is_ok());
    assert!(configure_timezone(&mut app).await.is_ok());
    assert!(configure_locale(&mut app).await.is_ok());
    assert!(packages().await);
    assert!(enable_services().await.is_ok());
    Ok(())
}
