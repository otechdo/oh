#![allow(clippy::multiple_crate_versions)]

use inquire::{prompt_confirmation, MultiSelect, Password, Select, Text};
use regex::Regex;
use std::collections::HashMap;
use std::fs::{remove_dir_all, remove_file, File};
use std::io;
use std::io::{read_to_string, BufRead};
use std::path::Path;
use std::process::{exit, Command, ExitCode};
use tabled::builder::Builder;
use tabled::settings::Style;
use tabled::{Table, Tabled};

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
    locales: Vec<String>,
    packages: Vec<String>,
    root: HashMap<bool, String>,
    users: Vec<Users>,
    users_table: Vec<Users>,
    success: i32,
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
        Self {
            locales: Vec::new(),
            packages: Vec::new(),
            root: HashMap::new(),
            users: Vec::new(),
            users_table: Vec::new(),
            success: 0,
        }
    }
}
impl Arch {
    #[must_use]
    pub fn new() -> Self {
        Self {
            locales: Vec::new(),
            packages: Vec::new(),
            root: HashMap::new(),
            users: Vec::new(),
            users_table: Vec::new(),
            success: 0,
        }
    }

    ///
    /// # Panics
    ///
    pub fn dotfiles(&mut self) -> &mut Self {
        let dot = prompt_confirmation("Clone a dotfiles repository ?").unwrap();
        let mut cmds: Vec<String> = Vec::new();
        if dot {
            if Path::new("/tmp/dotfiles").exists() {
                remove_dir_all("/tmp/dotfiles").expect("failed to remove the tmp dir");
            }
            let repo = Text::new("Enter repository url : ")
                .with_help_message("Url must be a git repository")
                .prompt()
                .unwrap();
            assert!(Command::new("git")
                .arg("clone")
                .arg("--quiet")
                .arg(repo.as_str())
                .arg("/tmp/dotfiles")
                .spawn()
                .unwrap()
                .wait()
                .unwrap()
                .success());
            loop {
                let cmd = Text::new("Please enter a bash command : ")
                    .with_help_message("Command will be executed in your repository")
                    .prompt()
                    .unwrap();
                cmds.push(cmd);
                match prompt_confirmation("Continue ?") {
                    Ok(true) => continue,
                    Ok(false) | Err(_) => break,
                }
            }
            for cmd in &cmds {
                let collection: Vec<&str> = cmd.split_whitespace().collect();
                assert!(Command::new("bash")
                    .args(collection)
                    .current_dir("/tmp/dotfiles")
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

    fn install_package(&mut self, pkgs: &[String]) -> &mut Self {
        for pkg in pkgs {
            assert!(Command::new("paru")
                .arg("-S")
                .arg("--noconfirm")
                .arg(pkg.as_str())
                .spawn()
                .unwrap()
                .wait()
                .unwrap()
                .success());
        }
        self
    }

    pub fn quit(&mut self) -> ExitCode {
        exit(self.success);
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
        let l = MultiSelect::new("Choose your system locale : ", locales)
            .with_help_message("Locale for the system")
            .prompt()
            .expect("Failed to get locales");
        if l.is_empty() {
            self.choose_locale()
        } else {
            for locale in &l {
                self.locales.push(locale.to_string());
            }
            self
        }
    }

    ///
    ///
    /// # Panics
    ///
    /// if failed to remove file
    ///
    pub fn choose_packages(&mut self) -> &mut Self {
        if Path::new("/tmp/eywa/pkgs").exists() {
            remove_file("/tmp/eywa/pkgs").expect("failed to remove file");
        }
        assert!(Command::new("make")
            .arg("pkgs")
            .current_dir("/tmp/eywa")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        let p = MultiSelect::new("Select packages : ", parse_file_lines("/tmp/eywa/pkgs"))
            .with_help_message("Packages to install on the system")
            .prompt()
            .expect("Failed to get packages");
        if p.is_empty() {
            self.choose_packages()
        } else {
            for x in &p {
                self.packages.push(x.to_string());
            }
            self
        }
    }

    fn create_users(&mut self) -> &mut Self {
        for user in &self.users {
            if user.sudoers {
                assert!(Command::new("useradd")
                    .arg("-m")
                    .arg("-g")
                    .arg("wheel")
                    .arg("-p")
                    .arg(&user.password)
                    .arg(&user.name)
                    .arg("-s")
                    .arg(&user.shell)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap()
                    .success());
            } else {
                assert!(Command::new("useradd")
                    .arg("-m")
                    .arg("-p")
                    .arg(&user.password)
                    .arg(&user.name)
                    .arg("-s")
                    .arg(&user.shell)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap()
                    .success());
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
                self.convert(&self.locales.clone()),
                self.convert(&self.packages.clone()),
                self.convert_users(self.users_table.clone())
            )
        );
        match prompt_confirmation("Run installation ? ") {
            Ok(true) => self
                .install_package(&self.packages.clone())
                .create_users()
                .quit(),
            Ok(false) | Err(_) => exit(1),
        }
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
        .choose_locale()
        .choose_packages()
        .dotfiles()
        .configure_root()
        .configure_users()
        .run()
}
