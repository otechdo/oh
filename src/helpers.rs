use inquire::MultiSelect;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::process::{exit, Command, ExitCode};
const DEBIAN_MANAGER: &str = "apt-get";
const FEDORA_MANAGER: &str = "dnf5";
const ARCH_MANAGER: &str = "yay";


#[must_use]
pub fn fedora() -> bool {
    Path::new("/etc/fedora-release").exists()
}
#[must_use]
pub fn debian() -> bool {
    Path::new("/etc/debian-release").exists()
}
#[must_use]
pub fn arch() -> bool {
    Path::new("/etc/arch-release").exists()
}

///
/// # Panics
///
#[must_use]
pub fn install(pkgs: &[String]) -> ExitCode {
    if arch() {
        for pkg in pkgs {
            if pkg.contains("arch") || pkg.contains("-S") {
                continue;
            }
            assert!(
                exec(
                    "sh",
                    &[
                        "-c",
                        format!("{ARCH_MANAGER} -S --noconfirm {pkg}").as_str()
                    ]
                ),
                "{}",
                format!("Failed to install {pkg}").as_str()
            );
            assert!(notifme::Notification::new()
                .app("arch")
                .summary(format!("{pkg} Installed").as_str())
                .body(format!("{pkg} has been installed successfully").as_str())
                .timeout(5)
                .send());
        }
        exit(0);
    }

    if fedora() {
        for pkg in pkgs {
            assert!(
                exec(
                    "sh",
                    &[
                        "-c",
                        format!("sudo {FEDORA_MANAGER} install -y {pkg}").as_str()
                    ]
                ),
                "{}",
                format!("Failed to install {pkg}").as_str()
            );
            assert!(notifme::Notification::new()
                .app("arch")
                .summary(format!("{pkg} Installed").as_str())
                .body(format!("{pkg} has been installed successfully").as_str())
                .timeout(5)
                .send());
        }
        exit(0);
    }

    if debian() {
        for pkg in pkgs {
            assert!(
                exec(
                    "sh",
                    &[
                        "-c",
                        format!("sudo {DEBIAN_MANAGER} install -y {pkg}").as_str()
                    ]
                ),
                "{}",
                format!("Failed to install {pkg}").as_str()
            );
            assert!(notifme::Notification::new()
                .app("arch")
                .summary(format!("{pkg} Installed").as_str())
                .body(format!("{pkg} has been installed successfully").as_str())
                .timeout(5)
                .send());
        }
        exit(0);
    }
    println!("Os not supported");
    exit(1);
}
///
/// # Panics
///
#[must_use]
pub fn uninstall(pkgs: &[String]) -> ExitCode {
    if arch() {
        for pkg in pkgs {
            if pkg.contains("arch") || pkg.contains("-S") {
                continue;
            }
            assert!(
                exec(
                    "sh",
                    &[
                        "-c",
                        format!("{ARCH_MANAGER} -Rns --noconfirm {pkg}").as_str()
                    ]
                ),
                "{}",
                format!("Failed to install {pkg}").as_str()
            );
            assert!(notifme::Notification::new()
                .app("arch")
                .summary(format!("{pkg} Installed").as_str())
                .body(format!("{pkg} has been installed successfully").as_str())
                .timeout(5)
                .send());
        }
        exit(0);
    }

    if fedora() {
        for pkg in pkgs {
            assert!(
                exec(
                    "sh",
                    &[
                        "-c",
                        format!("sudo {FEDORA_MANAGER} remove -y {pkg}").as_str()
                    ]
                ),
                "{}",
                format!("Failed to install {pkg}").as_str()
            );
            assert!(notifme::Notification::new()
                .app("arch")
                .summary(format!("{pkg} Installed").as_str())
                .body(format!("{pkg} has been installed successfully").as_str())
                .timeout(5)
                .send());
        }
        exit(0);
    }

    if debian() {
        for pkg in pkgs {
            assert!(
                exec(
                    "sh",
                    &[
                        "-c",
                        format!("sudo {DEBIAN_MANAGER} remove -y {pkg}").as_str()
                    ]
                ),
                "{}",
                format!("Failed to install {pkg}").as_str()
            );
            assert!(notifme::Notification::new()
                .app("arch")
                .summary(format!("{pkg} Installed").as_str())
                .body(format!("{pkg} has been installed successfully").as_str())
                .timeout(5)
                .send());
        }
        exit(0);
    }
    println!("Os not supported");
    exit(1);
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).expect("failed to open filename");
    io::BufReader::new(file).lines()
}

fn parse_file_lines(filename: &str) -> Vec<String> {
    let mut file_lines: Vec<String> = Vec::new();
    read_lines(filename).for_each(|line| match line {
        Ok(l) => {
            // perform some logic here...
            file_lines.push(l);
        }
        Err(x) => println!("{x}"),
    });
    file_lines
}
///
/// # Panics
///
#[must_use]
pub fn pkgs() -> Vec<String> {
    if arch() {
        if Path::new("/tmp/pkgs").exists() {
            return parse_file_lines("/tmp/pkgs");
        }
        assert!(exec(
            "sh",
            &["-c", "sudo pacman -Sl core | cut -d ' ' -f 2 > pkgs"]
        ));
        assert!(exec(
            "sh",
            &["-c", "sudo pacman -Sl extra | cut -d ' ' -f 2 >> pkgs"]
        ));
        assert!(exec(
            "sh",
            &["-c", "sudo pacman -Sl multilib | cut -d ' ' -f 2 >> pkgs"]
        ));
        assert!(exec("sh", &["-c", "sudo pacman -Sg >> pkgs"]));
        assert!(exec("sh", &["-c", "yay -Sl aur | cut -d ' ' -f 2 >> pkgs"]));
        assert!(exec("sh", &["-c", "sudo install -m 644 pkgs /tmp/pkgs"]));
        assert!(exec("sh", &["-c", "rm pkgs"]));
        return parse_file_lines("/tmp/pkgs");
    }

    if fedora() {
        if Path::new("/tmp/pkgs").exists() {
            return parse_file_lines("/tmp/pkgs");
        }
        assert!(exec(
            "sh",
            &[
                "-c",
                "sudo {FEDORA_MANAGER} --list core | cut -d ' ' -f 2 > pkgs"
            ]
        ));
        assert!(exec(
            "sh",
            &["-c", "sudo pacman -Sl extra | cut -d ' ' -f 2 >> pkgs"]
        ));
        assert!(exec(
            "sh",
            &["-c", "sudo pacman -Sl multilib | cut -d ' ' -f 2 >> pkgs"]
        ));
        assert!(exec("sh", &["-c", "sudo pacman -Sg >> pkgs"]));
        assert!(exec("sh", &["-c", "yay -Sl aur | cut -d ' ' -f 2 >> pkgs"]));
        assert!(exec("sh", &["-c", "sudo install -m 644 pkgs /tmp/pkgs"]));
        assert!(exec("sh", &["-c", "rm pkgs"]));
        return parse_file_lines("/tmp/pkgs");
    }
    vec![]
}
///
/// # Panics
///
#[must_use]
pub fn install_packages() -> ExitCode {
    if arch() || debian() || fedora() {
        return install(
            &MultiSelect::new("Select packages to install : ", pkgs())
                .prompt()
                .unwrap(),
        );
    }
    println!("Os not supported");
    exit(1);
}
///
/// # Panics
///
#[must_use]
pub fn uninstall_packages() -> ExitCode {
    if arch() || debian() || fedora() {
        return uninstall(
            &MultiSelect::new("Select packages to uninstall : ", pkgs())
                .prompt()
                .unwrap(),
        );
    }
    println!("Os not supported");
    exit(1);
}
