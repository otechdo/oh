use std::fs::{read_to_string, File};
use std::io::Error;
use std::process::Command;
use inquire::MultiSelect;
use crate::utils::confirm;

pub fn socket_running() -> Vec<String>
{
    update_systemctl_cache();
    let mut services:Vec<String> = Vec::new();
    let x = read_to_string("/tmp/sockets-running").expect("failed to read /tmp/sockets_running");
    for line in x.lines() {
        services.push(line.to_string());
    }
    services
}

pub fn services_running() -> Vec<String>
{
    update_systemctl_cache();
    let mut services:Vec<String> = Vec::new();
    let x = read_to_string("/tmp/services-running").expect("failed to read /tmp/services-running");
    for line in x.lines() {
        services.push(line.to_string());
    }
    services
}


pub fn services_disabled() -> Vec<String>
{
    update_systemctl_cache();
    let mut services:Vec<String> = Vec::new();
    let x = read_to_string("/tmp/services-disabled").expect("failed to read /tmp/services-disabled");
    for line in x.lines() {
        services.push(line.to_string());
    }
    services
}

pub fn socket_disabled() -> Vec<String>
{
    update_systemctl_cache();
    let mut services:Vec<String> = Vec::new();
    let x = read_to_string("/tmp/sockets-disabled").expect("failed to read /tmp/sockets-disabled");
    for line in x.lines() {
        services.push(line.to_string());
    }
    services
}

pub fn all_disabled() -> Vec<String>
{
    update_systemctl_cache();
    let mut services:Vec<String> = Vec::new();
    services.append(&mut services_disabled().to_vec());
    services.append(&mut socket_disabled().to_vec());
    services
}

pub fn all_running() -> Vec<String>
{
    update_systemctl_cache();
    let mut services:Vec<String> = Vec::new();
    services.append(&mut services_running().to_vec());
    services.append(&mut socket_running().to_vec());
    services
}
pub fn is_running(service:String)->bool
{
    services_running().contains(&service)
}

pub fn is_disabled(service:String)->bool
{
    services_disabled().contains(&service)
}

pub fn update_systemctl_cache()
{
    assert!(Command::new("systemctl").arg("list-units").arg("--type=socket").arg("--state=running").stdout(File::create("/tmp/sockets-running").expect("failed to create /tmp/sockets-running")).spawn().expect("failed to spawn /tmp/sockets-running").wait().expect("failed to wait for /tmp/sockets-running").success());
    assert!(Command::new("systemctl").arg("list-units").arg("--type=socket").arg("--state=disabled").stdout(File::create("/tmp/sockets-disabled").expect("failed to create /tmp/sockets-disabled")).spawn().expect("failed to spawn /tmp/sockets-disabled").wait().expect("failed to wait for /tmp/sockets-disabled").success());
    assert!(Command::new("systemctl").arg("list-units").arg("--type=service").arg("--state=running").stdout(File::create("/tmp/services-running").expect("failed to create /tmp/services-running")).spawn().expect("failed to spawn /tmp/services-running").wait().expect("failed to wait for /tmp/services-running").success());
    assert!(Command::new("systemctl").arg("list-units").arg("--type=service").arg("--state=disabled").stdout(File::create("/tmp/services-disabled").expect("failed to create /tmp/services-disabled")).spawn().expect("failed to spawn /tmp/services-disabled").wait().expect("failed to wait for /tmp/services-disabled").success());
}

pub async fn enable_services()-> Result<(),Error>
{
    loop {
        let services = MultiSelect::new("Select services to enable :",all_disabled()).prompt().expect("");
        for service in services {
            assert!(Command::new("systemctl").arg("enable").arg(service).spawn().expect("failed to enable service").wait().expect("failed to wait service").success());
        }
        if confirm("Activate another services ?").await.eq(&false) {
            break;
        }
    }
    Ok(())
}