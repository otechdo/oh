use crate::os::Os;
use std::io::Error;
use std::process::{Command, Stdio};

#[cfg(feature = "archlinux")]
pub async fn reflector(app: &mut Os) -> Result<(), Error> {
    assert!(Command::new("reflector")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-c")
        .arg(app.mirrors.country.as_str())
        .arg("--sort")
        .arg(app.mirrors.sort.as_str())
        .arg("--save")
        .arg(app.mirrors.save_at.as_str())
        .arg("-p")
        .arg(app.mirrors.protocol.as_str())
        .current_dir("/tmp")
        .spawn()
        .expect("missing reflector")
        .wait()
        .expect("")
        .success());
    Ok(())
}
