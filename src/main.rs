#![allow(clippy::multiple_crate_versions)]

use inquire::Password;
use inquire::{prompt_confirmation, Confirm, MultiSelect, Select, Text};
use std::env::args;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command, ExitCode};
use std::time::Instant;
use tabled::builder::Builder;
use tabled::{settings::Style, Table};

const VERSION: &str = "1.0.0";
const SELECT_PROFILES: &str = "Select a profiles bellow in the list : ";
const SELECT_PACKAGES: &str = "Select a packages bellow in the list : ";
const SELECT_TIMEZONE: &str = "Select a timezone bellow in the list : ";
const SELECT_LOCALES: &str = "Select a system locale bellow in the list : ";
const SELECT_KEYMAP: &str = "Select a system keymap bellow in the list : ";
const TYPE_HOSTNAME: &str = "Type your desired system hostname : ";
const GO_NEWS: &str = "Do you want go on the oh news ? : ";
const GO_AUR: &str = "Do you want go on the aur : ";
const GO_MAN: &str = "Do you want go on the oh man pages : ";
const GO_FORUM: &str = "Do you want go on the oh forum ? : ";
const GO_WIKI: &str = "Do you want go on the wiki ? : ";
const KEYMAP: &str = "/usr/share/oh/conf/keymaps";
const LOCALES: &str = "/usr/share/oh/conf/locales";
const PACKAGES: &str = "/usr/share/oh/cache/packages";
const TIME_ZONES: &str = "/usr/share/oh/conf/timezones";
const PROFILES: &str = "/usr/share/oh/profiles";
const ROOT_SERVICES_DISABLED: &str = "/usr/share/oh/services/disabled";
const ROOT_SERVICES_ENABLED: &str = "/usr/share/oh/services/enabled";
const COUNTRIES: &str = "/usr/share/oh/conf/countries";
const BOOT: &str = "/usr/share/applications/oh/boot";
const SHELLS: &str = "/usr/share/applications/oh/shells";

pub struct Arch {
    pub locales: Vec<String>,
    pub services: Vec<String>,
    pub packages: Vec<String>,
    pub users: Vec<Users>,
    pub timezone: String,
    pub keymap: String,
    pub hostname: String,
    pub boot: String,
    pub begin: Instant,
}

#[derive(Clone)]
pub struct Users {
    name: String,
    password: String,
    shell: String,
    sudoers: bool,
}

impl Users {
    #[must_use]
    pub const fn new(name: String, password: String, shell: String, sudoers: bool) -> Self {
        Self {
            name,
            password,
            shell,
            sudoers,
        }
    }
}

#[must_use]
fn exec(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .expect("failed to execute cmd")
        .success()
}

///
/// # Panics
///
fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file: File = File::open(filename).expect("failed to open filename");
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
///  # Panics
///
fn add_users() -> Vec<Users> {
    let mut users: Vec<Users> = Vec::new();
    if Confirm::new("Add users")
        .with_default(false)
        .prompt()
        .unwrap()
        .eq(&true)
    {
        loop {
            let name = Text::new("New user name : ").prompt().unwrap();
            let shell = Select::new("New user shell : ", parse_file_lines(SHELLS))
                .prompt()
                .unwrap();
            let password = Password::new("New user password : ").prompt().unwrap();
            let sudoers = Confirm::new("User can administrate the system : ")
                .with_default(false)
                .prompt()
                .unwrap();

            users.push(Users::new(name, password, shell, sudoers));
            if Confirm::new("Add a new user ? : ")
                .with_default(false)
                .prompt()
                .unwrap()
            {
                continue;
            }
            break;
        }
        return users;
    }
    users
}

///
///  # Panics
///
fn choose_packages() -> Vec<String> {
    let mut pkgs: Vec<String> = Vec::new();
    let mut builder = Builder::new();
    loop {
        let packages: Vec<String> = MultiSelect::new(SELECT_PACKAGES, parse_file_lines(PACKAGES))
            .prompt()
            .unwrap();
        if packages.is_empty() {
            continue;
        }
        for p in &packages {
            builder.push_record([p.to_string()]);
            pkgs.push(p.to_string());
        }
        println!("\n{}\n", builder.clone().build().with(Style::modern()));
        if Confirm::new("Add packages ?")
            .with_default(false)
            .prompt()
            .unwrap()
        {
            continue;
        }
        return pkgs;
    }
}

///
/// # Panics
///
fn remove_packages(packages: &[String]) -> i32 {
    for pkg in packages {
        if pkg.contains("oh") || pkg.contains("-R") {
            continue;
        }
        assert!(
            exec("sh", &["-c", format!("paru -Rns {pkg}").as_str()]),
            "{}",
            format!("Failed to install the {pkg} package").as_str()
        );
    }
    0
}

///
/// # Panics
///
fn install_profile(profile: &str) -> bool {
    println!("{}", format!("Installing the {profile} profile").as_str());
    assert!(Command::new("wget")
        .arg("-q")
        .arg(
            format!("https://raw.githubusercontent.com/otechdo/arch/main/arch/profiles/{profile}")
                .as_str()
        )
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    assert!(
        exec(
            "sh",
            &[
                "-c",
                format!("xargs -d '\n' -a {profile} paru --needed -Syu").as_str()
            ]
        ),
        "{}",
        format!("failed to install {profile}").as_str()
    );
    std::fs::remove_file(profile).expect("failed to remove profile file");
    true
}

///
/// # Panics
///
fn install_profiles() -> i32 {
    let mut builder = Builder::new();
    let profiles: Vec<String> = MultiSelect::new(SELECT_PROFILES, parse_file_lines(PROFILES))
        .prompt()
        .unwrap();
    if profiles.is_empty() {
        return install_profiles();
    }
    if profiles.first().unwrap().eq("@none") {
        println!("Bye");
        return 0;
    }
    for p in &profiles {
        builder.push_record([p.to_string()]);
    }
    println!("\n{}\n", builder.build().with(Style::modern()));
    if Confirm::new("Install profiles : ")
        .with_default(false)
        .prompt()
        .unwrap()
        .eq(&true)
    {
        for profile in &profiles {
            if Confirm::new(format!("Install the {profile} profile ? ").as_str())
                .with_default(true)
                .prompt()
                .unwrap()
            {
                assert!(
                    install_profile(profile),
                    "{}",
                    format!("Failed to install the {profile} profile")
                );
            }
        }
        return 0;
    }
    1
}

fn choose_keymap() -> String {
    loop {
        let keymap: String = Select::new(SELECT_KEYMAP, parse_file_lines(KEYMAP))
            .prompt()
            .unwrap();

        if keymap.is_empty() {
            continue;
        }
        if Confirm::new(format!("Use keymap {keymap} : ").as_str())
            .with_default(false)
            .prompt()
            .unwrap()
            .eq(&true)
        {
            return keymap;
        }
    }
}

fn install_keymap(keymap: &str) -> i32 {
    let mut k: File = File::create("vconsole.conf").expect("failed to create the keymap file");
    k.write_all(format!("KEYMAP={keymap}\nXKBLAYOUT={keymap}\n").as_bytes())
        .expect("failed to write data");
    k.sync_all().expect("failed to sync to disk");
    k.sync_data().expect("failed save to disk");

    assert!(exec(
        "sh",
        &["-c", "sudo install -m 644 vconsole.conf /etc/vconsole.conf"]
    ));
    assert!(exec("sh", &["-c", "sudo rm vconsole.conf"]));
    0
}

fn enable_services(services: &[String]) -> i32 {
    for service in services {
        assert!(
            exec(
                "sh",
                &["-c", format!("sudo systemctl enable {service}").as_str()]
            ),
            "{}",
            format!("Failed to enable {service} service").as_str()
        );
    }
    cache()
}

fn disabled_services(services: &[String]) -> i32 {
    for service in services {
        assert!(
            exec(
                "sh",
                &[
                    "-c",
                    format!("sudo systemctl disable --now {service}").as_str()
                ]
            ),
            "{}",
            format!("Failed to disable {service} service").as_str()
        );
    }
    cache()
}

fn install_locales(locales: &[String]) -> i32 {
    for locale in locales {
        assert!(exec(
            "sh",
            &[
                "-c",
                format!("sudo sed -i 's/#{locale} UTF-8/{locale} UTF-8/g' /etc/locale.gen")
                    .as_str()
            ]
        ));
    }
    assert!(exec("sh", &["-c", "sudo locale-gen"]));
    0
}

fn choose_hostname() -> String {
    loop {
        let hostname: String = Text::new(TYPE_HOSTNAME).prompt().unwrap();
        if hostname.is_empty() {
            continue;
        }
        if Confirm::new(format!("Use hostname {hostname} : ").as_str())
            .with_default(false)
            .prompt()
            .unwrap()
            .eq(&true)
        {
            return hostname;
        }
    }
}

fn choose_locales() -> Vec<String> {
    loop {
        let locales: Vec<String> = MultiSelect::new(SELECT_LOCALES, parse_file_lines(LOCALES))
            .prompt()
            .unwrap();

        if locales.is_empty() {
            continue;
        }

        if Confirm::new(
            format!(
                "Set LANG={} LOCALES={locales:?} ?",
                locales.first().unwrap()
            )
            .as_str(),
        )
        .with_default(false)
        .prompt()
        .unwrap()
        .eq(&true)
        {
            return locales;
        }
    }
}

fn choose_timezone() -> String {
    let mut builder = Builder::new();
    loop {
        let zone: String = Select::new(SELECT_TIMEZONE, parse_file_lines(TIME_ZONES))
            .prompt()
            .unwrap();

        if zone.is_empty() {
            continue;
        }
        builder.push_record([zone.to_string()]);
        println!("\n{}\n", builder.clone().build().with(Style::modern()));
        if Confirm::new("Use timezone ? ")
            .with_default(false)
            .prompt()
            .unwrap()
            .eq(&true)
        {
            return zone;
        }
        builder.clear();
    }
}

fn choose_boot() -> String {
    loop {
        let loader: String = Select::new("Select a boot loader", parse_file_lines(BOOT))
            .prompt()
            .unwrap();

        if loader.is_empty() {
            continue;
        }
        if Confirm::new(format!("Use boot loader {loader} : ").as_str())
            .with_default(false)
            .prompt()
            .unwrap()
            .eq(&true)
        {
            return loader;
        }
    }
}

fn install_language(lang: &str) -> i32 {
    let mut locale = File::create("locale.conf").expect("failed to create the locale file");
    locale
        .write_all(
            format!("LANG={lang}\nLC_COLLATE=C\nLANGUAGE={lang}\nLC_TIME={lang}",).as_bytes(),
        )
        .expect("failed to write data");
    locale.sync_all().expect("failed to sync to disk");
    locale.sync_data().expect("failed save to disk");

    assert!(exec(
        "sh",
        &["-c", "sudo install -m 644 locale.conf /etc/locale.conf"]
    ));
    assert!(exec("sh", &["-c", "rm locale.conf"]));
    0
}

fn choose_enabled_services() -> Vec<String> {
    let mut builder = Builder::new();
    let mut services: Vec<String> = Vec::new();
    services.append(&mut parse_file_lines(ROOT_SERVICES_DISABLED));
    loop {
        let services: Vec<String> =
            MultiSelect::new("Select services to disabled : ", services.clone())
                .prompt()
                .unwrap();

        if services.is_empty() {
            continue;
        }
        for s in &services {
            builder.push_record([s.to_string()]);
        }
        println!("\n{}\n", builder.clone().build().with(Style::modern()));
        if Confirm::new("Enabled services ? ")
            .with_default(false)
            .prompt()
            .unwrap()
            .eq(&true)
        {
            return services;
        }
        builder.clear();
    }
}

fn choose_disabled_services() -> Vec<String> {
    let mut builder = Builder::new();
    let mut services: Vec<String> = Vec::new();
    services.append(&mut parse_file_lines(ROOT_SERVICES_ENABLED));
    loop {
        let services: Vec<String> =
            MultiSelect::new("Select all unit to enable : ", services.clone())
                .prompt()
                .unwrap();

        if services.is_empty() {
            continue;
        }
        for s in &services {
            builder.push_record([s.to_string()]);
        }
        println!("\n{}\n", builder.clone().build().with(Style::modern()));
        if Confirm::new("Enabled services ? ")
            .with_default(false)
            .prompt()
            .unwrap()
            .eq(&true)
        {
            return services;
        }
        builder.clear();
    }
}
///
/// # Panics
///
fn man() -> i32 {
    if Confirm::new(GO_MAN).with_default(true).prompt().unwrap() {
        assert!(
            exec("sh", &["-c", "w3m man.archlinux.org"]),
            "Failed to navigate on man pages"
        );
    }
    0
}
///
/// # Panics
///
fn wiki() -> i32 {
    if Confirm::new(GO_WIKI).with_default(true).prompt().unwrap() {
        assert!(exec("sh", &["-c", "w3m wiki.archlinux.org"]));
    }
    0
}

///
/// # Panics
///
fn news() -> i32 {
    if Confirm::new(GO_NEWS).with_default(true).prompt().unwrap() {
        assert!(exec("sh", &["-c", "w3m archlinux.org/news"]));
    }
    0
}

///
/// # Panics
///
fn aur() -> i32 {
    if Confirm::new(GO_AUR).with_default(true).prompt().unwrap() {
        assert!(
            exec("sh", &["-c", "w3m aur.archlinux.org"]),
            "Failed to navigate on aur website"
        );
    }
    0
}
///
/// # Panics
///
fn upgrade() -> i32 {
    assert!(
        exec("sh", &["-c", "paru -Syu && flatpak update"]),
        "Failed to update the system"
    );
    0
}

fn help() -> i32 {
    println!(
        "\narch -i --setup Setup a new arch\n
arch -R --uninstall Uninstall all selected packages\n
arch -S --install Install all selected packages\n
arch -M --mirrors Update the arch mirrors\n
arch -C --check Check for available arch update\n
arch -d --deps Install all selected packages as deps\n
arch -u --update Update the system\n
arch -a --aur Show aur packages\n
arch -s --search Search a package\n
arch -v --version Display version information\n
arch -d --download-updates Dowload all arch updates\n
arch -h --help Display the help message\n
arch -x --cancel Cancel the reboot\n
arch -U --upgrade Update arch and reboot after 5 min\n
arch -c --cache Refresh arch packages cache\n
arch -n --news Show aur news\n
arch -f --forum Show arch forum\n
arch -m --man --woman  Show arch manpages\n
arch --new-config Clean arch and reconfigure\n
arch -w --wiki Show arch wiki\n"
    );
    1
}

fn cache_package() -> i32 {
    assert!(exec(
        "sh",
        &["-c", "sudo pacman -Sl core | cut -d ' ' -f 2 > packages"]
    ));
    assert!(exec(
        "sh",
        &["-c", "sudo pacman -Sl extra | cut -d ' ' -f 2 >> packages"]
    ));
    assert!(exec(
        "sh",
        &[
            "-c",
            "sudo pacman -Sl multilib | cut -d ' ' -f 2 >> packages"
        ]
    ));
    assert!(exec("sh", &["-c", "sudo pacman -Sg >> packages"]));
    assert!(exec(
        "sh",
        &["-c", "paru -Sl aur | cut -d ' ' -f 2 >> packages"]
    ));
    assert!(exec(
        "sh",
        &[
            "-c",
            format!("sudo install -m 644 packages {PACKAGES}").as_str()
        ]
    ));
    assert!(exec("sh", &["-c", "rm packages"]));
    0
}

fn cache_services() -> i32 {
    assert!(exec(
        "sh",
        &["-c", "systemctl list-unit-files --type=service --state=disabled | cut -d ' ' -f 1 > root_disabled"]
    ));
    assert!(exec(
        "sh",
        &["-c", "systemctl list-unit-files --type=socket --state=disabled | cut -d ' ' -f 1 >> root_disabled"]
    ));

    assert!(exec(
        "sh",
        &[
            "-c",
            format!("sudo install -m 644 root_disabled {ROOT_SERVICES_DISABLED}").as_str()
        ]
    ));
    assert!(exec(
        "sh",
        &[
            "-c",
            "systemctl list-unit-files --type=service --state=enabled | cut -d ' ' -f 1 > root_enabled"
        ]
    ));
    assert!(exec(
        "sh",
        &[
            "-c",
            "systemctl list-unit-files --type=socket --state=enabled | cut -d ' ' -f 1 >> root_enabled"
        ]
    ));

    assert!(exec(
        "sh",
        &[
            "-c",
            format!("sudo install -m 644 root_enabled {ROOT_SERVICES_ENABLED}").as_str()
        ]
    ));
    assert!(exec("sh", &["-c", "rm  root_enabled root_disabled"]));
    0
}
fn cache() -> i32 {
    assert!(cache_package().eq(&0));
    assert!(cache_services().eq(&0));
    0
}

///
/// # Panics
///
fn install_packages(packages: &[String]) -> i32 {
    for pkg in packages {
        assert!(
            exec("sh", &["-c", format!("paru -S {pkg}").as_str()]),
            "{}",
            format!("Failed to install the {pkg} package").as_str()
        );
    }
    if Confirm::new("Services must be enabled ?")
        .with_default(false)
        .prompt()
        .unwrap()
        .eq(&true)
    {
        assert!(cache().eq(&0), "Failed to update cache");
        return enable_services(&choose_disabled_services());
    }
    0
}

///
/// # Panics
///
fn install_dependencies(packages: &[String]) -> i32 {
    for pkg in packages {
        if pkg.contains("oh") || pkg.contains("-d") {
            continue;
        }
        assert!(
            exec("sh", &["-c", format!("paru -S {pkg} --asdeps").as_str()]),
            "{}",
            format!("Failed to install the {pkg} dependencies").as_str()
        );
    }
    0
}

///
/// # Panics
///
fn configure_mirrors() -> i32 {
    let country = Select::new("Please enter your country : ", parse_file_lines(COUNTRIES))
        .prompt()
        .unwrap();
    let sort = Select::new(
        "Select a sort option : ",
        vec!["age", "country", "delay", "rate", "score"],
    )
    .prompt()
    .unwrap();
    let protocol = Select::new(
        "Select a protocol option : ",
        vec!["ftp", "http", "https", "rsync"],
    )
    .prompt()
    .unwrap();

    if country.is_empty() || sort.is_empty() || protocol.is_empty() {
        return configure_mirrors();
    }

    if Confirm::new(format!("Set the mirror server to the {country} country : ").as_str())
        .with_default(true)
        .prompt()
        .unwrap()
        && Confirm::new(format!("Sort the mirrorlist by {sort} : ").as_str())
            .with_default(true)
            .prompt()
            .unwrap()
        && Confirm::new(format!("Use the protocol {protocol} : ").as_str())
            .with_default(true)
            .prompt()
            .unwrap()
    {
        assert!(exec(
            "sh",
            &[
                "-c",
                format!(
                    "sudo reflector --sort {sort} -c {country} --save /etc/pacman.d/mirrorlist -p {protocol}"
                )
                .as_str()
            ],
        ),"Failed to regenerate mirrors");
        assert!(exec("sh", &["-c", "paru -Syu"]), "Failed to update mirrors");
        return 0;
    }
    configure_mirrors()
}

///
/// # Panics
///
fn install_boot(boot: &str) -> i32 {
    if boot.eq("grub") {
        assert!(exec("sh", &["-c", "sudo mkdir -p /boot/grub"]));
        assert!(
            exec("sh", &["-c", "sudo grub-mkconfig -o /boot/grub/grub.cfg"]),
            "Failed to generate grub config"
        );
        assert!(exec("sh", &["-c", "sudo grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id oh --recheck"]),"Failed to install grub menu");
        return 0;
    }
    if boot.eq("systemd-boot") {
        assert!(exec("sh", &["-c", "sudo mkdir -p /boot/grub"]));
        assert!(
            exec("sh", &["-c", "sudo grub-mkconfig -o /boot/grub/grub.cfg"]),
            "Failed to generate grub config"
        );
        assert!(exec("sh", &["-c", "sudo grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id oh --recheck"]),"Failed to install grub menu");
        return 0;
    }
    if boot.eq("efistub") {
        assert!(exec("sh", &["-c", "sudo mkdir -p /boot/grub"]));
        assert!(
            exec("sh", &["-c", "sudo grub-mkconfig -o /boot/grub/grub.cfg"]),
            "Failed to generate grub config"
        );
        assert!(exec("sh", &["-c", "sudo grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id oh --recheck"]),"Failed to install grub menu");
        return 0;
    }
    if boot.eq("syslinux") {
        assert!(exec("sh", &["-c", "sudo mkdir -p /boot/grub"]));
        assert!(
            exec("sh", &["-c", "sudo grub-mkconfig -o /boot/grub/grub.cfg"]),
            "Failed to generate grub config"
        );
        assert!(exec("sh", &["-c", "sudo grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id oh --recheck"]),"Failed to install grub menu");
        return 0;
    }
    1
}

///
/// # Panics
///
fn remove_profile() -> i32 {
    let profiles: Vec<String> = MultiSelect::new(SELECT_PROFILES, parse_file_lines(PROFILES))
        .prompt()
        .unwrap();
    if profiles.is_empty() {
        return remove_profile();
    }
    if profiles.first().unwrap().eq("@none") {
        println!("Bye");
        return 0;
    }
    for profile in &profiles {
        if Confirm::new(format!("Remove the {profile} profile").as_str())
            .with_default(true)
            .prompt()
            .unwrap()
        {
            assert!(Command::new("wget")
                .arg("-q")
                .arg(
                    format!(
                    "https://raw.githubusercontent.com/otechdo/arch/main/arch/profiles/{profile}"
                )
                    .as_str()
                )
                .current_dir(".")
                .spawn()
                .unwrap()
                .wait()
                .unwrap()
                .success());
            assert!(
                exec(
                    "sh",
                    &[
                        "-c",
                        format!("xargs -d '\n' -a {profile} paru -Rdd").as_str()
                    ]
                ),
                "{}",
                format!("Failed to remove {profile}").as_str()
            );
            std::fs::remove_file(profile).expect("Failed to remove profile file");
        }
    }
    0
}

///
/// # Panics
///
fn forums() -> i32 {
    if Confirm::new(GO_FORUM).with_default(true).prompt().unwrap() {
        assert!(exec("sh", &["-c", "w3m bbs.archlinux.org"]));
    }
    0
}

impl Arch {
    #[must_use]
    pub fn new(begin: Instant) -> Self {
        Self {
            locales: Vec::new(),
            services: Vec::new(),
            packages: Vec::new(),
            users: Vec::new(),
            timezone: String::new(),
            keymap: String::new(),
            hostname: String::new(),
            boot: String::new(),
            begin,
        }
    }

    ///
    /// # Panics
    ///
    pub fn update(&mut self) -> &mut Self {
        assert!(configure_mirrors().eq(&0), "Failed to configure mirrors");
        let _ = exec("sh", &["-c", "paru -Syyu"]);
        self
    }
    pub fn set_locales(&mut self, locales: &[String]) -> &mut Self {
        self.locales.clear();
        self.locales = locales.to_vec();
        self
    }

    pub fn set_services(&mut self, services: &[String]) -> &mut Self {
        self.services.clear();
        self.services = services.to_vec();
        self
    }

    pub fn set_packages(&mut self, packages: &[String]) -> &mut Self {
        self.packages.clear();
        self.packages = packages.to_vec();
        self
    }
    pub fn set_timezone(&mut self, zone: &str) -> &mut Self {
        self.timezone.clear();
        self.timezone.push_str(zone);
        self
    }
    pub fn set_keymap(&mut self, keymap: &str) -> &mut Self {
        self.keymap.clear();
        self.keymap.push_str(keymap);
        self
    }
    pub fn set_hostname(&mut self, hostname: &str) -> &mut Self {
        self.hostname.clear();
        self.hostname.push_str(hostname);
        self
    }
    pub fn set_boot(&mut self, boot: &str) -> &mut Self {
        self.boot.clear();
        self.boot.push_str(boot);
        self
    }
    pub fn set_users(&mut self, users: &[Users]) -> &mut Self {
        self.users.clear();
        self.users = users.to_vec();
        self
    }

    ///
    /// # Panics
    ///
    pub fn dotfiles(&mut self) -> &mut Self {
        let dot = prompt_confirmation("Clone a personal repository ?").unwrap();
        let mut commands: Vec<String> = Vec::new();
        if dot {
            if Path::new("dot").exists() {
                assert!(exec("sh", &["-c", "rm -rf dot"]));
            }
            let repo = Text::new("Enter repository url : ")
                .with_help_message("Url must be a git repository")
                .prompt()
                .unwrap();
            assert!(
                exec(
                    "sh",
                    &["-c", format!("git clone --quiet {repo} dot").as_str()]
                ),
                "Failed to download your dot repository"
            );

            loop {
                let cmd = Text::new("Please enter a bash command : ")
                    .prompt()
                    .unwrap();
                commands.push(cmd);
                match prompt_confirmation("Add a new command ? ") {
                    Ok(true) => {
                        println!("{}", Table::new(commands.clone()).with(Style::modern()));
                        continue;
                    }
                    Ok(false) | Err(_) => break,
                }
            }
            for cmd in &commands {
                let collection: Vec<&str> = cmd.split_whitespace().collect();
                assert!(Command::new("bash")
                    .args(collection)
                    .current_dir("dot")
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap()
                    .success());
            }
            assert!(
                exec("sh", &["-c", "rm -rf dot"]),
                "Failed to remove dot directory"
            );
            return self;
        }
        self
    }

    pub fn quit(&mut self, t: &str) -> ExitCode {
        println!("{t}");
        exit(0);
    }

    ///
    /// # Panics
    ///
    pub fn check_network(&mut self) -> &mut Self {
        println!("Checking network...");
        assert!(exec(
            "sh",
            &["-c", "ping -4c4 www.microsoft.com > /dev/null 2> /dev/null"]
        ));
        self
    }

    ///
    /// # Panics
    ///
    pub fn create_users(&mut self) -> &mut Self {
        for user in &self.users {
            assert!(
                exec(
                    "sh",
                    &[
                        "-c",
                        format!(
                            "sudo useradd -m -U -p {} {} -s {}",
                            user.password, user.name, user.shell
                        )
                        .as_str()
                    ]
                ),
                "Failed to create the new user"
            );
            if user.sudoers {
                assert!(
                    exec(
                        "sh",
                        &[
                            "-c",
                            format!(
                                "sudo echo '{} ALL=(ALL) ALL' > /etc/sudoers.d/{} ",
                                user.name, user.name
                            )
                            .as_str()
                        ]
                    ),
                    "Failed to create the new user"
                );
            }
        }
        self
    }

    ///
    /// # Panics
    ///
    pub fn run(&mut self) -> i32 {
        assert!(cache().eq(&0), "Failed to update the cache");
        let run = Confirm::new("Run installation ? ")
            .with_default(true)
            .prompt()
            .unwrap();
        assert!(news().eq(&0), "Failed to parse input");
        assert!(wiki().eq(&0), "Failed to parse input");
        assert!(forums().eq(&0), "Failed to parse input");
        if run {
            assert!(
                install_keymap(self.keymap.as_str()).eq(&0),
                "Failed to configure keymap"
            );
            assert!(
                configure_hostname(self.hostname.as_str()).eq(&0),
                "Failed to configure hostname"
            );
            assert!(
                install_locales(&self.locales).eq(&0),
                "Failed to configure locales"
            );
            assert!(
                install_language(self.locales.first().unwrap().to_string().as_str()).eq(&0),
                "Failed to configure lang"
            );
            assert!(
                install_timezone(self.timezone.as_str()).eq(&0),
                "Failed to configure lang"
            );
            assert!(install_profiles().eq(&0), "Failed to install a profile");
            assert!(
                install_packages(&choose_packages()).eq(&0),
                "Failed to install a profile"
            );
            assert!(
                install_boot(self.boot.as_str()).eq(&0),
                "Failed to install a profile"
            );
            assert!(
                enable_services(&self.services).eq(&0),
                "Failed to install profile"
            );
            println!(
                "Execution time : {} min",
                self.begin.elapsed().as_secs() * 60
            );
            return 0;
        }
        println!(
            "Execution time : {} secs",
            Instant::now().duration_since(self.begin).as_secs()
        );
        1
    }
}

fn configure_hostname(hostname: &str) -> i32 {
    assert!(
        exec(
            "sh",
            &["-c", format!("echo {hostname} > hostname").as_str()],
        ),
        "Failed to save hostname"
    );
    assert!(
        exec("sh", &["-c", "sudo install -m 644 hostname /etc/hostname"],),
        "Failed to install hostname"
    );
    assert!(
        exec("sh", &["-c", "rm hostname"]),
        "Failed to remove tmp hostname file"
    );
    0
}
///
/// # Panics
///
fn installer() -> i32 {
    let mut arch = Arch::new(Instant::now());
    let start = Confirm::new("Start installation")
        .with_default(false)
        .prompt()
        .unwrap();
    if start.eq(&false) {
        println!("Bye");
        exit(1);
    }
    let license = Table::new([fs::read_to_string("/usr/share/licenses/oh/LICENSE")
        .expect("No license has been found")
        .as_str()])
    .with(Style::modern())
    .to_string();
    println!("{license}\n",);
    let accepting = Confirm::new("Accept license ?")
        .with_default(false)
        .prompt()
        .unwrap();
    if accepting.eq(&false) {
        println!("Bye");
        exit(1);
    }
    arch.check_network()
        .update()
        .set_locales(&choose_locales())
        .set_keymap(choose_keymap().as_str())
        .set_hostname(choose_hostname().as_str())
        .set_users(&add_users())
        .set_boot(choose_boot().as_str())
        .set_timezone(choose_timezone().as_str())
        .run()
}

fn install_timezone(timezone: &str) -> i32 {
    assert!(
        exec(
            "sh",
            &[
                "-c",
                format!("sudo ln -sfv /usr/share/zoneinfo/{timezone} /etc/localtime").as_str()
            ]
        ),
        "Failed to set timezone"
    );
    0
}

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        exit(upgrade());
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--setup") || args.get(1).unwrap().eq("-i") {
        exit(installer());
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--cache") {
        exit(cache());
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--install") {
        exit(install_packages(&choose_packages()));
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--remove-profiles") {
        exit(remove_profile());
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--install-profiles") {
        exit(install_profiles());
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--reconfigure") {
        exit(installer())
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--uninstall") {
        exit(remove_packages(&choose_packages()))
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--install-dependencies")
        || args.get(1).unwrap().eq("-d")
    {
        exit(install_dependencies(&choose_packages()))
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--install-profiles") {
        exit(install_profiles())
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--man") || args.get(1).unwrap().eq("-m") {
        exit(man())
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--wiki") || args.get(1).unwrap().eq("-w") {
        exit(wiki())
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--forum") || args.get(1).unwrap().eq("-f") {
        exit(forums())
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--aur") || args.get(1).unwrap().eq("-a") {
        exit(aur())
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--version") || args.get(1).unwrap().eq("-v") {
        println!("Arch version : {VERSION}");
        exit(0);
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--configure-time-zone")
        || args.get(1).unwrap().eq("-t")
    {
        exit(install_timezone(&choose_timezone()));
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--enable-services") {
        exit(enable_services(&choose_disabled_services()));
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--disable-services") {
        exit(disabled_services(&choose_enabled_services()));
    }
    exit(help());
}
