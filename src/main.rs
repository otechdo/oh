#![allow(clippy::multiple_crate_versions)]

use inquire::{prompt_confirmation, MultiSelect, Password, Select, Text};
use regex::Regex;
use std::collections::HashMap;
use std::fs::{remove_file, File};
use std::io;
use std::io::{read_to_string, BufRead};
use std::path::Path;
use std::process::{exit, Command, ExitCode};
use tabled::builder::Builder;
use tabled::settings::Style;
use tabled::{Table, Tabled};

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
pub struct Arch {
    locales: String,
    packages: Vec<String>,
    root: HashMap<bool, String>,
    users: Vec<Users>,
    users_table: Vec<Users>,
    timezone: String,
    keymap: String,
}

#[derive(Tabled, Clone)]
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

impl Default for Arch {
    #[must_use]
    fn default() -> Self {
        if Path::new("eywa").exists() {
            assert!(exec("sh", &["-c", "sudo rm -rf eywa"]));
        }
        if !Path::new("eywa").exists() {
            assert!(exec("sh", &["-c", "sudo mkdir -p eywa"]));
            assert!(exec("sh", &["-c", "sudo chmod -R 777 eywa"]));
        }
        Self {
            locales: String::new(),
            packages: Vec::new(),
            root: HashMap::new(),
            users: Vec::new(),
            users_table: Vec::new(),
            timezone: String::new(),
            keymap: String::new(),
        }
    }
}
impl Arch {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    ///
    /// # Panics
    ///
    pub fn dotfiles(&mut self) -> &mut Self {
        let dot = prompt_confirmation("Clone a dotfiles repository ?").unwrap();
        let mut cmds: Vec<String> = Vec::new();
        if dot {
            if Path::new("eywa/dotfiles").exists() {
                assert!(exec("sh", &["-c", "sudo rm -rf eywa/dotfiles"]));
            }
            let repo = Text::new("Enter repository url : ")
                .with_help_message("Url must be a git repository")
                .prompt()
                .unwrap();
            assert!(exec(
                "sh",
                &[
                    "-c",
                    format!("git clone --quiet {repo} eywa/dotfiles").as_str()
                ]
            ));
            loop {
                let cmd = Text::new("Please enter a bash command : ")
                    .prompt()
                    .unwrap();
                cmds.push(cmd);
                match prompt_confirmation("Add a new command ? ") {
                    Ok(true) => continue,
                    Ok(false) | Err(_) => break,
                }
            }
            for cmd in &cmds {
                let collection: Vec<&str> = cmd.split_whitespace().collect();
                assert!(Command::new("bash")
                    .args(collection)
                    .current_dir("eywa/dotfiles")
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap()
                    .success());
            }
            return self;
        }
        self
    }

    ///
    ///  # Panics
    ///
    fn install_package(&mut self, packages: &[String]) -> &mut Self {
        for pkg in packages {
            assert!(exec(
                "sh",
                &["-c", format!("paru -S --noconfirm {pkg}").as_str()]
            ));
        }
        self
    }

    ///
    /// # Panics
    ///
    pub fn quit(&mut self) -> ExitCode {
        assert!(exec("sh", &["-c", "sudo rm -rf eywa"]));
        assert!(exec("sh", &["-c", "sudo rm locale.conf"]));
        assert!(exec("sh", &["-c", "sudo rm vconsole.conf"]));
        exit(self.configure_boot().enable_services());
    }

    ///
    /// # Panics
    ///
    pub fn enable_services(&mut self) -> i32 {
        assert!(exec(
            "sh",
            &["-c", "sudo systemctl enable NetworkManager.service"]
        ));
        assert!(exec(
            "sh",
            &[
                "-c",
                "sudo systemctl enable NetworkManager-wait-online.service"
            ]
        ));
        0
    }
    ///
    /// # Panics
    ///
    /// if failed to get locale
    ///
    pub fn choose_locale(&mut self) -> &mut Self {
        let mut locales: Vec<String> = Vec::new();
        let text =
            read_to_string(File::open("/etc/locale.gen").expect("failed to open locale file"))
                .expect("failed to get file content");
        let re = Regex::new(r"[a-z]{2}_[A-Z]{2}[.][A-Z]{3}-[0-9]").unwrap(); // \d means digit
        for mat in re.find_iter(text.as_str()) {
            locales.push(mat.as_str().to_string());
        }
        let l = Select::new("Choose your system locale : ", locales)
            .with_help_message("Locale for the system")
            .prompt()
            .expect("Failed to get locales");
        if l.is_empty() {
            self.choose_locale()
        } else {
            self.locales.push_str(l.as_str());
            self
        }
    }

    ///
    /// # Panics
    ///
    pub fn choose_keymap(&mut self) -> &mut Self {
        self.keymap
            .push_str(Text::new("Enter your keymap : ").prompt().unwrap().as_str());
        if self.keymap.is_empty() {
            self.choose_keymap()
        } else {
            self
        }
    }

    ///
    /// # Panics
    ///
    pub fn configure_timezone(&mut self) -> &mut Self {
        assert!(exec(
            "sh",
            &[
                "-c",
                format!(
                    "sudo ln -sf /usr/share/zoneinfo/{} /etc/localtime",
                    self.timezone
                )
                .as_str()
            ]
        ));
        self
    }

    ///
    /// # Panics
    ///
    pub fn check_network(&mut self) -> &mut Self {
        assert!(exec("sh", &["-c", "ping -4c4 archlinux.org"]));
        self
    }

    ///
    /// # Panics
    ///
    pub fn configure_keymap(&mut self) -> &mut Self {
        assert!(exec(
            "sh",
            &[
                "-c",
                format!("echo 'KEYMAP={}' > vconsole.conf", self.keymap).as_str()
            ]
        ));
        assert!(exec(
            "sh",
            &["-c", "sudo install -m 644 vconsole.conf /etc/vconsole.conf"]
        ));
        self
    }

    ///
    /// # Panics
    ///
    pub fn configure_locale(&mut self) -> &mut Self {
        assert!(exec(
            "sh",
            &[
                "-c",
                format!("sudo echo \"LANG={}\" > locale.conf", self.locales).as_str()
            ]
        ));
        assert!(exec(
            "sh",
            &["-c", "sudo install -m 644 locale.conf /etc/locale.conf"]
        ));

        assert!(exec(
            "sh",
            &[
                "-c",
                format!("sudo sed -i 's/#{} UTF-8/{} UTF-8/g' /etc/locale.gen",self.locales,self.locales).as_str()
            ]
        ));
        self
    }

    ///
    ///
    /// # Panics
    ///
    /// if failed to remove file
    ///
    pub fn choose_packages(&mut self) -> &mut Self {
        if Path::new("eywa/pkgs").exists() {
            remove_file("eywa/pkgs").expect("failed to remove file");
        }
        assert!(exec(
            "sh",
            &["-c", "sudo pacman -Sl core | cut -d ' ' -f 2 > eywa/pkgs"]
        ));
        assert!(exec(
            "sh",
            &["-c", "sudo pacman -Sl extra | cut -d ' ' -f 2 >> eywa/pkgs"]
        ));
        assert!(exec(
            "sh",
            &[
                "-c",
                "sudo pacman -Sl multilib | cut -d ' ' -f 2 >> eywa/pkgs"
            ]
        ));
        assert!(exec(
            "sh",
            &["-c", "paru --list  aur | cut -d ' ' -f 2 >> eywa/pkgs"]
        ));
        loop {
            let p = MultiSelect::new("Select packages : ", parse_file_lines("eywa/pkgs"))
                .with_help_message("Packages to install on the system")
                .prompt()
                .expect("Failed to get packages");
            if p.is_empty() {
                return self.choose_packages();
            } else {
                for x in &p {
                    self.packages.push(x.to_string());
                }

                match prompt_confirmation("Add package ? ") {
                    Ok(true) => continue,
                    Ok(false) | Err(_) => break,
                }
            }
        }
        self
    }

    ///
    /// # Panics
    ///
    fn configure_boot(&mut self) -> &mut Self {
        assert!(exec("sh", &["-c", "sudo mkdir -p /boot/grub"]));
        assert!(exec(
            "sh",
            &["-c", "sudo grub-mkconfig -o /boot/grub/grub.cfg"]
        ));
        assert!(exec("sh", &["-c", "sudo grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id arch --recheck"]));
        self
    }

    ///
    /// # Panics
    ///
    pub fn create_users(&mut self) -> &mut Self {
        for user in &self.users {
            if user.sudoers {
                assert!(exec(
                    "sh",
                    &[
                        "-c",
                        format!(
                            "sudo useradd -m -g wheel -p {} {} -s {}",
                            user.password, user.name, user.shell
                        )
                        .as_str()
                    ]
                ));
            } else {
                assert!(exec(
                    "sh",
                    &[
                        "-c",
                        format!(
                            "sudo useradd -m -p {} {} -s {}",
                            user.password, user.name, user.shell
                        )
                        .as_str()
                    ]
                ));
            }
        }
        self
    }
    pub fn convert(&mut self, x: &[String]) -> String {
        let mut builder = Builder::new();
        for i in x {
            builder.push_record([i.as_str()]);
        }
        builder.build().with(Style::modern_rounded()).to_string()
    }

    pub fn convert_users(&mut self, users: Vec<Users>) -> String {
        Table::new(users).with(Style::modern_rounded()).to_string()
    }
    pub fn run(&mut self) -> ExitCode {
        println!(
            "{}",
            format_args!(
                "\nLocale:\n{}\nPackages:\n{}\n{}",
                self.convert(&[self.locales.to_string()]),
                self.convert(&self.packages.clone()),
                self.convert_users(self.users_table.clone())
            )
        );
        match prompt_confirmation("Run installation ? ") {
            Ok(true) => self
                .install_package(&self.packages.clone())
                .create_users()
                .configure_timezone()
                .configure_locale()
                .configure_keymap()
                .quit(),
            Ok(false) | Err(_) => exit(1),
        }
    }

    ///
    /// # Panics
    ///
    pub fn pacman_keys(&mut self) -> &mut Self {
        assert!(exec("sh", &["-c", "pacman-key --refresh-keys"]));
        self
    }
    ///
    /// # Panics
    ///
    pub fn configure_users(&mut self) -> &mut Self {
        loop {
            let name = Text::new("New Username : ")
                .with_help_message("New username")
                .prompt()
                .unwrap();
            let shell = Select::new(
                format!("{name}'s shell : ").as_str(),
                vec![
                    "sh",
                    "bash",
                    "git-shell",
                    "rbash",
                    "fish",
                    "zsh",
                    "tcsh",
                    "closh",
                    "elvish",
                    "ion",
                    "murex",
                    "oh",
                    "xonsh",
                    "nushell",
                ],
            )
            .prompt()
            .unwrap();

            if !name.is_empty() {
                let password = Password::new(format!("{name}'s password : ").as_str())
                    .prompt()
                    .unwrap();

                let sudoers = match prompt_confirmation(
                    format!("{name}'s user can administrate the system : ").as_str(),
                ) {
                    Ok(false) | Err(_) => false,
                    Ok(true) => true,
                };
                self.users.push(Users::new(
                    name.to_string(),
                    password.to_string(),
                    format!("/usr/bin/{shell}"),
                    sudoers,
                ));
                self.users_table.push(Users::new(
                    name.to_string(),
                    "********".to_string(),
                    shell.to_string(),
                    sudoers,
                ));
                match prompt_confirmation("Add a new user ?") {
                    Ok(true) => continue,
                    Ok(false) | Err(_) => break,
                }
            }
        }
        self
    }

    ///
    /// # Panics
    ///
    pub fn choose_timezone(&mut self) -> &mut Self {
        self.timezone = Text::new("Please enter your timezone : ").prompt().unwrap();
        if self.timezone.is_empty() {
            self.choose_timezone()
        } else {
            self
        }
    }

    ///
    /// # Panics
    ///
    pub fn configure_mirrors(&mut self) -> &mut Self {
        let country = Text::new("Please enter your country ? : ")
            .prompt()
            .unwrap();

        let mut parallel = Text::new("Please enter the pacman parallel downloads size : ")
            .prompt()
            .unwrap();
        if parallel.is_empty() {
            parallel.push('5');
        }
        assert!(exec(
            "sh",
            &[
                "-c",
                format!(
                    "sudo reflector --sort delay -c {country} --save /etc/pacman.d/mirrorlist -p https"
                )
                .as_str()
            ],
        ));
        assert!(exec("sh",&["-c",format!("sudo sed -i 's/#ParallelDownloads = 5/ParallelDownloads = {parallel}/g' /etc/pacman.conf").as_str()]));
        self
    }

    ///
    /// # Panics
    ///
    pub fn configure_root(&mut self) -> &mut Self {
        let root = match prompt_confirmation("Enable root user ? ") {
            Ok(true) => true,
            Ok(false) | Err(_) => false,
        };
        if root {
            let password = Password::new("root's password : ").prompt().unwrap();
            assert!(self.root.insert(root, password).is_none());
        } else {
            assert!(self.root.insert(root, String::new()).is_none());
        }
        self
    }
}

fn main() -> ExitCode {
    Arch::new()
        .check_network()
        .configure_mirrors()
        .choose_locale()
        .choose_timezone()
        .choose_packages()
        .run()
}
