pub mod helpers;
use crate::helpers::{exec, install, install_packages, uninstall_packages};
use std::env::args;
use std::process::{exit, ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();

    if args.len() == 2 && args.get(1).unwrap().eq("-S") {
        return install(&args);
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--install") {
        return install_packages();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--uninstall") {
        return uninstall_packages();
    }
    if args.len() == 5 && args.get(1).unwrap().eq("--add") {
        assert!(
            exec(
                "sh",
                &[
                    "-c",
                    format!(
                        "toolbox create --distro {} --release {} -c {}",
                        args.get(2).unwrap(),
                        args.get(3).unwrap(),
                        args.get(4).unwrap()
                    )
                    .as_str()
                ]
            ),
            "Failed to create container"
        );
        exit(0);
    }
    if args.len() == 4 && args.get(1).unwrap().eq("--run") {
        assert!(
            exec(
                "sh",
                &[
                    "-c",
                    format!(
                        "toolbox run -c {} {}",
                        args.get(2).unwrap(),
                        args.get(3).unwrap(),
                    )
                    .as_str()
                ]
            ),
            "Failed to create container"
        );

        exit(0);
    }
    if args.len() == 4 && args.get(1).unwrap().eq("--add") {
        assert!(
            exec(
                "sh",
                &[
                    "-c",
                    format!(
                        "toolbox create --distro {} --release {}",
                        args.get(2).unwrap(),
                        args.get(3).unwrap(),
                    )
                    .as_str()
                ]
            ),
            "Failed to create container"
        );

        exit(0);
    }

    if args.len() == 3 && args.get(1).unwrap().eq("--del") {
        assert!(
            exec(
                "sh",
                &[
                    "-c",
                    format!("toolbox rm {} ", args.get(2).unwrap()).as_str()
                ]
            ),
            "Failed to remove container"
        );
        exit(0);
    }

    if args.len() == 3 && args.get(1).unwrap().eq("--add-from") {
        assert!(
            exec(
                "sh",
                &[
                    "-c",
                    format!("toolbox create --image {}", args.get(2).unwrap()).as_str()
                ]
            ),
            "Failed to create container"
        );
        exit(0);
    }

    if args.len() == 3 && args.get(1).unwrap().eq("--use") {
        assert!(
            exec(
                "sh",
                &[
                    "-c",
                    format!("toolbox enter {}", args.get(2).unwrap()).as_str()
                ]
            ),
            "Failed to enter inside the container"
        );
        exit(0);
    }

    if args.len() == 2 && args.get(1).unwrap().eq("--list") {
        assert!(
            exec("sh", &["-c", "toolbox list"]),
            "Failed to enter inside the container"
        );
        exit(0);
    }
    if args.len() == 3 && args.get(1).unwrap().eq("--stop") {
        assert!(
            exec("sh", &["-c", format!("podman container stop {}",args.get(2).unwrap()).as_str()]),
            "Failed to stop the container"
        );
        exit(0);
    }
    exit(1);
}
