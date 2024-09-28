use std::fs::File;
use std::process::Command;

pub fn generate_pacman_cache()->bool
{
    Command::new("pacman").arg("-Sl").stdout(File::create("/tmp/packages").expect("fail to create file")).spawn().expect("failed to create the cache file").wait().unwrap().success()
}