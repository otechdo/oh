#[cfg(feature = "gentoo")]
pub mod stage;
#[cfg(feature = "gentoo")]
pub mod network;

#[cfg(feature = "gentoo")]
#[cfg(not(feature = "archlinux"))]
pub async fn install_gentoo(_expert: bool) -> Result<(), Error> {
    Ok(())
}
