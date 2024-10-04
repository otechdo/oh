use std::fs::read_to_string;
use std::io::Error;
use std::process::Command;

#[cfg(feature = "ask")]
use crate::utils::confirm;
#[cfg(feature = "ask")]
use inquire::MultiSelect;

pub fn socket_running() -> Vec<String> {
    assert!(update_systemctl_cache());
    let mut services: Vec<String> = Vec::new();
    let x = read_to_string("/tmp/sockets-running").expect("failed to read /tmp/sockets_running");
    for line in x.lines() {
        services.push(line.to_string());
    }
    services
}

pub fn services_running() -> Vec<String> {
    assert!(update_systemctl_cache());
    let mut services: Vec<String> = Vec::new();
    let x = read_to_string("/tmp/services-running").expect("failed to read /tmp/services-running");
    for line in x.lines() {
        services.push(line.to_string());
    }
    services
}

pub fn services_disabled() -> Vec<String> {
    assert!(update_systemctl_cache());
    let mut services: Vec<String> = Vec::new();
    let x =
        read_to_string("/tmp/services-disabled").expect("failed to read /tmp/services-disabled");
    for line in x.lines() {
        services.push(line.to_string());
    }
    services
}

pub fn socket_disabled() -> Vec<String> {
    assert!(update_systemctl_cache());
    let mut services: Vec<String> = Vec::new();
    let x = read_to_string("/tmp/sockets-disabled").expect("failed to read /tmp/sockets-disabled");
    for line in x.lines() {
        services.push(line.to_string());
    }
    services
}

pub fn all_disabled() -> Vec<String> {
    assert!(update_systemctl_cache());
    let mut services: Vec<String> = Vec::new();
    services.append(&mut services_disabled().to_vec());
    services.append(&mut socket_disabled().to_vec());
    services
}

pub fn all_running() -> Vec<String> {
    assert!(update_systemctl_cache());
    let mut services: Vec<String> = Vec::new();
    services.append(&mut services_running().to_vec());
    services.append(&mut socket_running().to_vec());
    services
}
pub fn is_running(service: String) -> bool {
    services_running().contains(&service)
}

pub fn is_disabled(service: String) -> bool {
    services_disabled().contains(&service)
}

pub fn update_systemctl_cache() -> bool {
    Command::new("update-services-cache")
        .spawn()
        .expect("failed to spawn update-services-cache")
        .wait()
        .expect("failed to wait for update-services-cache")
        .success()
}

pub async fn enable_services() -> Result<(), Error> {
    loop {
        let services = MultiSelect::new("Select services to enable :", all_disabled())
            .prompt()
            .expect("");
        for service in services {
            assert!(Command::new("systemctl")
                .arg("enable")
                .arg(service)
                .spawn()
                .expect("failed to enable service")
                .wait()
                .expect("failed to wait service")
                .success());
        }
        if confirm("Activate another services ?").await.eq(&false) {
            break;
        }
    }
    Ok(())
}
