use std::io::Error;
use crate::ai::gemini::ai;

pub mod network;
pub mod stage;

pub async fn install_gentoo() -> Result<(), Error> {
    ai().await;
    Ok(())
}
