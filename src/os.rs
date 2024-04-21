use inquire::prompt_confirmation;
use inquire::{Select, Text};
use regex as _;
use std::env::args;
use std::process::{exit, Command, ExitCode};
const IMAGES: [&str; 80] = [
    "bazzite-arch:latest",
    "bazzite-arch-gnome:latest",
    "fedora-toolbox:rawhide",
    "distrobox:latest",
    "ubi8/toolbox",
    "rhel-toolbox:latest",
    "rockylinux-toolbox:8",
    "rockylinux-toolbox:9",
    "rockylinux-toolbox:latest",
    "ubuntu-toolbox:16.04",
    "ubuntu-toolbox:18.04",
    "ubuntu-toolbox:20.04",
    "ubuntu-toolbox:22.04",
    "ubuntu-toolbox:latest",
    "almalinux:8",
    "almalinux:9",
    "alpine:3.15",
    "alpine:3.16",
    "alpine:latest",
    "amazonlinux:1",
    "amazonlinux:2",
    "amazonlinux:2023",
    "archlinux:latest",
    "centos:stream8",
    "centos:stream9",
    "centos:7",
    "wolfi-base:latest",
    "clearlinux:latest",
    "clearlinux:base",
    "docker:latest",
    "debian/eol:wheezy",
    "debian:buster-backports",
    "debian:bullseye-backports",
    "debian:bookworm-backports",
    "debian:stable-backports",
    "debian:testing",
    "debian:testing-backports",
    "debian:unstable",
    "linuxdeepin/apricot",
    "linuxdeepin/beige",
    "fedora:36",
    "fedora:37",
    "fedora:38",
    "fedora:39",
    "fedora:rawhide",
    "tage3:latest",
    "plasma:latest",
    "kali-rolling:latest",
    "mint21.1-amd64",
    "neurodebian:nd100",
    "leap:latest",
    "distrobox:latest",
    "tumbleweed:latest",
    "toolbox:latest",
    "oraclelinux:7",
    "oraclelinux:7-slim",
    "oraclelinux:8",
    "oraclelinux:8-slim",
    "oraclelinux:9",
    "oraclelinux:9-slim",
    "ubi7/ubi",
    "ubi8/ubi",
    "ubi8/ubi-init",
    "ubi8/ubi-minimal",
    "ubi9/ubi",
    "ubi9/ubi-init",
    "ubi9/ubi-minimal",
    "rockylinux:8",
    "rockylinux:8-minimal",
    "rockylinux:9",
    "rockylinux:latest",
    "sl:7",
    "steamos:latest",
    "ubuntu:14.04",
    "ubuntu:16.04",
    "ubuntu:18.04",
    "ubuntu:20.04",
    "ubuntu:22.04",
    "vso:main",
    "void-glibc-full:latest",
];

#[macro_export]
macro_rules! exec {
    ($program:expr,$args:expr) => {
        assert!(
            Command::new($program)
                .args($args)
                .spawn()
                .unwrap()
                .wait()
                .expect("failed to execute cmd")
                .success(),
            $program
        );
    };
}

fn upgrade() -> ExitCode {
    exec!("sh", &["-c", "distrobox upgrade --all --root"]);
    exit(0);
}

fn enter(name: &str) -> ExitCode {
    exec!(
        "sh",
        &["-c", format!("distrobox enter --root {name}").as_str()]
    );
    exit(0);
}

fn run(name: &str, cmd: &str) -> ExitCode {
    exec!(
        "sh",
        &[
            "-c",
            format!("distrobox enter --root {name} -- {cmd} ").as_str()
        ]
    );
    exit(0);
}

fn remove(name: &str) -> ExitCode {
    exec!(
        "sh",
        &["-c", format!("distrobox rm --root {name}").as_str()]
    );
    exit(0);
}

fn clean() -> ExitCode {
    exec!("sh", &["-c", "distrobox rm --force --all --root"]);
    exit(0);
}

fn stop_all() -> ExitCode {
    exec!("sh", &["-c", "distrobox stop  --root --force --all --yes"]);
    exit(0);
}

fn pause(name: &str) -> ExitCode {
    exec!(
        "sh",
        &["-c", format!("distrobox stop {name} --root --yes").as_str()]
    );
    exit(0);
}

fn create() -> ExitCode {
    loop {
        let image: &str = Select::new("Select an image to create : ", IMAGES.to_vec())
            .prompt()
            .unwrap();

        let name: &str = &Text::new("Enter the container name : ").prompt().unwrap();
        if name.is_empty() || image.is_empty() {
            continue;
        }
        let hostname: &str = &Text::new("Enter the container hostname : ")
            .prompt()
            .unwrap();
        if name.is_empty() || image.is_empty() || hostname.is_empty() {
            continue;
        }
        let isoled: bool = match prompt_confirmation("Isolate the container ? : ") {
            Ok(true) => true,
            Ok(false) | Err(_) => false,
        };

        let nvidia: bool =
            match prompt_confirmation("Integrate nvidia driver in the container ? : ") {
                Ok(true) => true,
                Ok(false) | Err(_) => false,
            };
        let e: bool = match prompt_confirmation("Enter in the container after the creation ? : ") {
            Ok(true) => true,
            Ok(false) | Err(_) => false,
        };

        if isoled {
            if nvidia {
                exec!(
                "sh",
                &[
                    "-c",
                    format!("distrobox create --image {image} --unshare-all --name {name} --hostname {hostname} --pull --nvidia --root").as_str()
                ]
            );
            } else {
                exec!(
                "sh",
                &[
                    "-c",
                    format!("distrobox create --image {image} --unshare-all --name {name} --hostname {hostname} --pull --root").as_str()
                ]
            );
            }
        } else {
            exec!(
                "sh",
                &[
                    "-c",
                    format!("distrobox create --image {image} --name {name} --hostname {hostname} --pull  --root").as_str()
                ]
            );
        }
        if e {
            return enter(name);
        }
        break;
    }
    exit(0);
}

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();

    if args.len() == 2 && args.get(1).unwrap().eq("--new") {
        return create();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--upgrade") {
        return upgrade();
    }
    if args.len() == 3 && args.get(1).unwrap().eq("--remove") {
        return remove(args.get(2).unwrap());
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--clean") {
        return clean();
    }
    if args.len() == 3 && args.get(1).unwrap().eq("--use") {
        return enter(args.get(2).unwrap());
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--list") {
        exec!("sh", &["-c", "distrobox list --root"]);
        exit(0);
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--stop") {
        return stop_all();
    }
    if args.len() == 3 && args.get(1).unwrap().eq("--pause") {
        return pause(args.get(2).unwrap());
    }
    if args.len() == 4 && args.get(1).unwrap().eq("--run") {
        return run(args.get(2).unwrap(), args.get(3).unwrap());
    }
    exit(1);
}
