#![allow(clippy::multiple_crate_versions)]

use ask_gemini::Gemini;
use clap::{Arg, ArgMatches, Command};
use inquire::{Confirm, Text};

fn oh() -> ArgMatches {
    Command::new("oh")
        .arg(
            Arg::new("distro")
                .long("distro")
                .help("Specifies the Linux distribution to install")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("bootloader")
                .long("bootloader")
                .help("Specifies the Linux distribution bootloader to install")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("boot")
                .long("boot")
                .help("Specifies the Linux boot partition")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("progress")
                .long("progress")
                .help("Set the progress output")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("root")
                .long("root")
                .help("Specifies the Linux root partition")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("swap")
                .long("swap")
                .help("Specifies the Linux swap partition")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("efi")
                .long("efi")
                .help("Specifies the Linux efi partition")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("home")
                .long("home")
                .help("Specifies the Linux home partition")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("boot-fs")
                .long("boot-fs")
                .help("Specifies the Linux boot partition filesystem")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("root-fs")
                .long("root-fs")
                .help("Specifies the Linux root partition filesystem")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("hostname")
                .long("hostname")
                .help("Specifies the Linux system hostname")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("group")
                .long("group")
                .help("Add user to given group")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("timezone")
                .long("timezone")
                .help("Configure the timezone")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("keymap")
                .long("keymap")
                .help("Configure the keymap")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("lang")
                .long("lang")
                .help("Configure the lang")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("locale")
                .long("locale")
                .help("Sets the default locale for the system, influencing date/time, number formatting, etc")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("bootloader-entries")
                .long("bootloader-entries")
                .help("Configures the bootloader to display specific entries")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("user")
                .long("user")
                .help("Configure the user")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("fullname")
                .long("fullname")
                .help("Configure the full username")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("needed")
                .long("needed")
                .help("Installs only packages that are not already present")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("shell")
                .long("shell")
                .help("Configure the shell for the user")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("editor")
                .long("editor")
                .help("Specifies the editor")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("aur-helper")
                .long("aur-helper")
                .help("Specifies the aur helper to use")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("aur")
                .long("aur")
                .help("Specifies the aur packages to install")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("groups")
                .long("groups")
                .help("Specifies the groups to install")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("packages")
                .long("packages")
                .help("Specifies the packages to install")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("enable-services")
                .long("enable-services")
                .help("Specifies the services to enable")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("enable-sockets")
                .long("enable-sockets")
                .help("Specifies the services to enable")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("arch")
                .long("arch")
                .help("Specifies the computer arch")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("target")
                .long("target")
                .help("Specifies the computer target")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("disable-root")
                .long("disable-root")
                .help("Option to suppress root user support")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("enable-root")
                .long("enable-root")
                .help("Option to enable root user")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("ask-password")
                .long("ask-password")
                .help("Ask users password")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("root-password")
                .long("root-password")
                .help("Set root password")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("user-password")
                .long("user-password")
                .help("Set user password")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("mirrorlist")
                .long("mirrorlist")
                .help("Configure the mirror url")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("country")
                .long("country")
                .help("Configure the mirror country")
                .required(false)
                .require_equals(false)
                .num_args(1),
        ).arg(
            Arg::new("color")
                .long("color")
                .help("Controls the use of colors in the console output")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("browser")
                .long("browser")
                .help("Install the specified browser to navigate on the internet")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("ai")
                .long("ai")
                .help("Specify the AI helper to use during the installation")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("ai-api-key")
                .long("ai-api-key")
                .help("Specify the API key for the AI helper if required")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("reflector")
                .long("reflector")
                .help("Automatically generates a mirror list based on your location and network speed")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("wifi-password")
                .long("wifi-password")
                .help("Used to connect to a Wi-Fi network during the installation")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("wifi-ssid")
                .long("wifi-ssid")
                .help("Used to connect to a Wi-Fi network during the installation")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("ip")
                .long("ip")
                .help("Sets a static IP address for the system")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )   .arg(
        Arg::new("dhcp")
            .long("dhcp")
            .help("Enables automatic network configuration using DHCP")
            .required(false)
            .require_equals(false)
            .num_args(1),
    )
        .arg(
            Arg::new("gateway")
                .long("gateway")
                .help("Sets the default gateway for network access")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("save-mirror-at")
                .long("save-mirror-at")
                .help("Saves the list of selected mirrors in the given path")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("script")
                .long("script")
                .help("Executes the given script after the base installation")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("noconfirm")
                .long("noconfirm")
                .help("Does not display confirmation prompts")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("logfile")
                .long("logfile")
                .help("Saves installation logs to a file")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
        Arg::new("reboot")
            .long("reboot")
            .help("Automatically reboot the system after installation")
            .required(false)
            .require_equals(false)
            .num_args(1),
    )  .arg(
        Arg::new("gfx-driver")
            .long("gfx-driver")
            .help("Installs the driver for graphics cards")
            .required(false)
            .require_equals(false)
            .num_args(1),
    )
        .arg(
            Arg::new("wifi-driver")
                .long("wifi-driver")
                .help("Installs the Broadcom wireless driver")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("audio-driver")
                .long("audio-driver")
                .help("Installs the sound driver")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("bluetooth")
                .long("bluetooth")
                .help("Enables Bluetooth support during installation")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("printer")
                .long("printer")
                .help("Installs CUPS (Common Unix Printing System) for printing support")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("3d-printer")
                .long("3d-printer")
                .help("Installs 3d printing support")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("virtualbox-guest-modules")
                .long("virtualbox-guest-modules")
                .help("Installs necessary modules for running Arch Linux as a VirtualBox guest")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("vmware-guest-modules")
                .long("vmware-guest-modules")
                .help("Installs necessary modules for running in a VMware guest")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("profile")
                .long("profile")
                .help("Installs necessary modules for running in a VMware guest")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("depots")
                .long("depots")
                .help("Repositories to be used to download packages")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("font")
                .long("font")
                .help("Specifies a default font to be installed for the system")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("no-upgrade")
                .long("no-upgrade")
                .help("Skips upgrading packages during the installation process")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("bootloader-timeout")
                .long("bootloader-timeout")
                .help("Sets the timeout (in seconds) for the bootloader menu")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("path")
                .long("path")
                .help("Sets the timeout (in seconds) for the bootloader menu")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("wish")
                .long("wish")
                .help("Expresses the preference to install")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("on-wish-failure")
                .long("on-wish-failure")
                .help("Expresses the preference to install secondary")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("on-wish-success")
                .long("on-wish-success")
                .help("Execute the given script after the base success installation")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("script-requirement")
                .long("script-requirement")
                .help("Execute the given script after the base success installation")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("no-reboot")
                .long("no-reboot")
                .help("Prevents the system from rebooting after installation")
                .required(false)
                .require_equals(false)
                .num_args(0),
        )
        .arg(
            Arg::new("skip-chroot")
                .long("skip-chroot")
                .help("Skips entering the chroot environment after the base installation, useful if you want to customize things further manually.")
                .required(false)
                .require_equals(false)
                .num_args(0),
        )
        .arg(
            Arg::new("chroot")
                .long("chroot")
                .help("Entering in the chroot environment after the base installation")
                .required(false)
                .require_equals(false)
                .num_args(0),
        )
        .arg(
            Arg::new("firmware")
                .long("firmware")
                .help("Installs the necessary firmware for certain hardware devices")
                .required(false)
                .require_equals(false)
                .num_args(1),
        )
        .arg(
            Arg::new("accept-mirrors")
                .long("accept-mirrors")
                .help("Limits the selection to mirrors using the given protocol")
                .required(false)
                .require_equals(false)
                .num_args(0),
        )
        .get_matches()
}
async fn ai()
{
    let gemini = Gemini::new(Some("AIzaSyDD4E07ashjDh4AZA7E-dD_OkXCLwnhF7Y"), None);
    loop {
        let prompt = Text::new("").prompt().unwrap_or_default();
        match gemini.ask(prompt.as_str()).await {
            Ok(response) => {
                assert!(std::process::Command::new("clear").spawn().expect("linux").wait().is_ok());
                let x = response.join("\n");
                println!("{x}");
            },
            Err(e) => eprintln!("Error: {e}"),
        }
        if Confirm::new("Quit ? :").with_default(false).prompt().unwrap() {
            break;
        }
    }
}
#[tokio::main]
async fn main() {
    #[cfg(feature = "ai")]
    ai().await;

    let _app = oh();




}
