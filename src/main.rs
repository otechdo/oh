#![allow(clippy::multiple_crate_versions)]

use inquire::{prompt_confirmation, Confirm, MultiSelect, Password, Select, Text};
use regex::Regex;
use std::collections::HashMap;
use std::env::args;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::{read_to_string, BufRead};
use std::path::Path;
use std::process::{exit, Command, ExitCode};
use tabled::builder::Builder;
use tabled::{settings::Style, Table};
const VERSION: &str = "1.0.0";

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
    lang: String,
    packages: Vec<String>,
    root: HashMap<bool, String>,
    users: Vec<Users>,
    user: String,
    users_table: Vec<Users>,
    timezone: String,
    keymap: String,
    hostname: String,
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

impl Default for Arch {
    #[must_use]
    fn default() -> Self {
        Self {
            locales: Vec::new(),
            lang: String::new(),
            packages: Vec::new(),
            root: HashMap::new(),
            users: Vec::new(),
            users_table: Vec::new(),
            timezone: String::new(),
            keymap: String::new(),
            hostname: String::new(),
            user: String::new(),
        }
    }
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

impl Arch {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    ///
    /// # Panics
    ///
    pub fn systemd(&mut self) -> &mut Self {
        assert!(
            exec(
                "sh",
                &["-c","wget -q https://raw.githubusercontent.com/otechdo/arch/main/arch/systemd/arch.service"]
            ),
            "Failed to download arch.service"
        );
        assert!(
            exec(
                "sh",
                &["-c","wget -q https://raw.githubusercontent.com/otechdo/arch/main/arch/systemd/arch.timer"]
            ),
            "Failed to download arch.timer"
        );
        assert!(
            exec(
                "sh",
                &[
                    "-c",
                    "sudo install -m 644 arch.timer /etc/systemd/system/arch.timer"
                ]
            ),
            "Failed to install arch.timer"
        );
        assert!(
            exec(
                "sh",
                &[
                    "-c",
                    "sudo install -m 644 arch.service /etc/systemd/system/arch.service"
                ]
            ),
            "Failed to install arch.service"
        );
        assert!(
            exec("sh", &["-c", "sudo systemctl enable arch.service"]),
            "Failed to enable arch.service"
        );
        assert!(
            exec("sh", &["-c", "sudo systemctl enable arch.timer"]),
            "Failed to enable arch.timer"
        );
        assert!(
            exec("sh", &["-c", "rm arch.service"]),
            "Failed to enable arch.service"
        );
        assert!(
            exec("sh", &["-c", "rm arch.timer"]),
            "Failed to enable arch.timer"
        );

        self
    }

    ///
    /// # Panics
    ///
    pub fn dotfiles(&mut self) -> &mut Self {
        let dot = prompt_confirmation("Clone a dotfiles repository ?").unwrap();
        let mut cmds: Vec<String> = Vec::new();
        if dot {
            if Path::new("dotfiles").exists() {
                assert!(exec("sh", &["-c", "rm -rf dotfiles"]));
            }
            let repo = Text::new("Enter repository url : ")
                .with_help_message("Url must be a git repository")
                .prompt()
                .unwrap();
            assert!(
                exec(
                    "sh",
                    &["-c", format!("git clone --quiet {repo} dotfiles").as_str()]
                ),
                "Failed to download your dotfiles repository"
            );

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
                    .current_dir("dotfiles")
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap()
                    .success());
            }
            assert!(
                exec("sh", &["-c", "rm -rf dotfiles"]),
                "Failed to remove dotefiles directory"
            );
            return self;
        }
        self
    }

    ///
    ///  # Panics
    ///
    fn install_package(&mut self) -> &mut Self {
        for pkg in &self.packages {
            assert!(
                exec("sh", &["-c", format!("paru -S {pkg}").as_str()]),
                "{}",
                format!("Failed to install the {pkg}").as_str()
            );
        }
        self
    }

    ///
    ///  # Panics
    ///
    fn install_dependencies(&mut self) -> &mut Self {
        for pkg in &self.packages {
            assert!(
                exec("sh", &["-c", format!("paru -S {pkg} --asdeps").as_str()]),
                "{}",
                format!("Failed to install {pkg} dependency").as_str()
            );
        }
        self
    }

    ///
    ///  # Panics
    ///
    fn remove_package(&mut self) -> &mut Self {
        for pkg in &self.packages {
            assert!(
                exec("sh", &["-c", format!("paru -Rns {pkg}").as_str()]),
                "{}",
                format!("Failed to remove {pkg} dependency").as_str()
            );
        }
        self
    }

    ///
    /// # Panics
    ///
    pub fn quit_installer(&mut self) -> ExitCode {
        exit(self.configure_boot().enable_services());
    }

    pub fn quit(&mut self, t: &str) -> ExitCode {
        println!("{t}");
        exit(0);
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
        assert!(exec("sh", &["-c", "sudo systemctl enable cups.service"]));
        0
    }

    ///
    /// # Panics
    ///
    pub fn choose_language(&mut self) -> &mut Self {
        let mut locales: Vec<String> = Vec::new();
        self.lang.clear();
        let text =
            read_to_string(File::open("/etc/locale.gen").expect("failed to open locale file"))
                .expect("failed to get file content");
        let re = Regex::new(r"[a-z]{2}_[A-Z]{2}[.][A-Z]{3}-[0-9]").unwrap(); // \d means digit
        for mat in re.find_iter(text.as_str()) {
            locales.push(mat.as_str().to_string());
        }
        let locale = Select::new("Choose your system language : ", locales.clone())
            .prompt()
            .expect("Failed to get locales");
        if locale.is_empty() {
            self.choose_language()
        } else {
            self.lang = locale;

            self
        }
    }

    ///
    /// # Panics
    ///
    pub fn choose_locales(&mut self) -> &mut Self {
        let mut locales: Vec<String> = Vec::new();
        self.locales.clear();
        let text =
            read_to_string(File::open("/etc/locale.gen").expect("failed to open locale file"))
                .expect("failed to get file content");
        let re = Regex::new(r"[a-z]{2}_[A-Z]{2}[.][A-Z]{3}-[0-9]").unwrap(); // \d means digit
        for mat in re.find_iter(text.as_str()) {
            locales.push(mat.as_str().to_string());
        }
        let locale = MultiSelect::new("Choose your system locales : ", locales.clone())
            .prompt()
            .expect("Failed to get locales");
        if locale.is_empty() {
            self.choose_locales()
        } else {
            self
        }
    }

    ///
    /// # Panics
    ///
    pub fn wiki(&mut self) -> &mut Self {
        assert!(exec("sh", &["-c", "w3m wiki.archlinux.org"]));
        self
    }

    ///
    /// # Panics
    ///
    pub fn news(&mut self) -> &mut Self {
        assert!(exec("sh", &["-c", "w3m archlinux.org/news"]));
        self
    }

    ///
    /// # Panics
    ///
    pub fn choose_keymap(&mut self) -> &mut Self {
        let keymap = Select::new(
            "Please enter your keymap : ",
            parse_file_lines("/tmp/keymaps"),
        )
        .prompt()
        .unwrap();
        if keymap.is_empty() {
            return self.choose_keymap();
        }
        self.keymap.clear();
        self.keymap.push_str(keymap.as_str());
        self
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
                    "sudo ln -svf /usr/share/zoneinfo/{} /etc/localtime",
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
    pub fn forums(&mut self) -> &mut Self {
        assert!(exec("sh", &["-c", "w3m bbs.archlinux.org"]));
        self
    }

    ///
    /// # Panics
    ///
    pub fn check_network(&mut self) -> &mut Self {
        println!("Checking network...");
        assert!(exec(
            "sh",
            &["-c", "ping -4c4 archlinux.org > /dev/null 2> /dev/null"]
        ));
        self
    }

    ///
    /// # Panics
    ///
    pub fn configure_keymap(&mut self) -> &mut Self {
        let mut keymap = File::create("vconsole.conf").expect("failed to cretae the keymap file");
        keymap
            .write_all(format!("KEYMAP={}\nXKBLAYOUT={}", self.keymap, self.keymap).as_bytes())
            .expect("failed to write data");
        keymap.sync_all().expect("failed to sync to disk");
        keymap.sync_data().expect("failed save to disk");

        assert!(exec(
            "sh",
            &["-c", "sudo install -m 644 vconsole.conf /etc/vconsole.conf"]
        ));

        assert!(exec("sh", &["-c", "sudo rm vconsole.conf"]));
        self
    }

    ///
    /// # Panics
    ///
    pub fn configure_locale(&mut self) -> &mut Self {
        let mut locale = File::create("locale.conf").expect("failed to cretae the locale file");
        locale
            .write_all(
                format!(
                    "LANG={}\nLC_COLLATE=C\nLANGUAGE={}\nLC_TIME={}",
                    self.lang, self.lang, self.lang
                )
                .as_bytes(),
            )
            .expect("failed to write data");
        locale.sync_all().expect("failed to sync to disk");
        locale.sync_data().expect("failed save to disk");

        assert!(exec(
            "sh",
            &["-c", "sudo install -m 644 locale.conf /etc/locale.conf"]
        ));
        assert!(exec("sh", &["-c", "rm locale.conf"]));

        for locale in &self.locales {
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
        self
    }

    ///
    ///
    /// # Panics
    ///
    /// if failed to remove file
    ///
    pub fn choose_packages(&mut self) -> &mut Self {
        if Path::new("/tmp/packages").exists() {
            self.packages.clear();
            assert!(self.packages.is_empty());
            loop {
                let p = MultiSelect::new("Select packages : ", parse_file_lines("/tmp/packages"))
                    .with_help_message("Packages to install on the system")
                    .prompt()
                    .expect("Failed to get packages");
                if p.is_empty() {
                    return self.choose_packages();
                }
                for x in &p {
                    self.packages.push(x.to_string());
                }

                match prompt_confirmation("Add package ? ") {
                    Ok(true) => continue,
                    Ok(false) | Err(_) => break,
                }
            }
            return self;
        }
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
            &["-c", "sudo install -m 644 packages /tmp/packages"]
        ));
        assert!(exec("sh", &["-c", "rm packages"]));
        self.choose_packages()
    }

    ///
    /// # Panics
    ///
    fn configure_boot(&mut self) -> &mut Self {
        assert!(exec("sh", &["-c", "sudo mkdir -p /boot/grub"]));
        assert!(
            exec("sh", &["-c", "sudo grub-mkconfig -o /boot/grub/grub.cfg"]),
            "Failed to generate grub config"
        );
        assert!(exec("sh", &["-c", "sudo grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id arch --recheck"]),"Failed to install grub menu");
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
    pub fn run(&mut self) -> ExitCode {
        let run = Confirm::new("Run installation ? ")
            .with_default(true)
            .prompt()
            .unwrap();
        if run {
            return self.install_package().create_users().quit_installer();
        }
        install()
    }

    ///
    /// # Panics
    ///
    pub fn configure_users(&mut self) -> &mut Self {
        let create = match prompt_confirmation("Create a new user ? : ") {
            Ok(true) => true,
            Ok(false) | Err(_) => false,
        };
        if create {
            loop {
                let name = Text::new("New Username : ")
                    .with_help_message("New username")
                    .prompt()
                    .unwrap();
                let shell = Select::new(
                    format!("{name}'s shell : ").as_str(),
                    parse_file_lines("/etc/shells"),
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
            return self;
        }
        self
    }

    ///
    /// # Panics
    ///
    pub fn choose_timezone(&mut self) -> &mut Self {
        let zone = Select::new(
            "Please enter your timezone : ",
            parse_file_lines("/tmp/timezones"),
        )
        .prompt()
        .unwrap();
        if zone.is_empty() {
            return self.choose_timezone();
        }
        self.timezone.clear();
        self.timezone.push_str(zone.as_str());
        self
    }

    ///
    /// # Panics
    ///
    pub fn configure_mirrors(&mut self) -> &mut Self {
        let country = Select::new(
            "Please enter your country : ",
            parse_file_lines("/tmp/countries"),
        )
        .prompt()
        .unwrap();

        if country.is_empty() {
            return self.configure_mirrors();
        }
        let confirm_mirror =
            Confirm::new(format!("Set your mirrorlist to the {country} country : ").as_str())
                .with_default(true)
                .prompt()
                .unwrap();
        if confirm_mirror {
            assert!(exec(
            "sh",
            &[
                "-c",
                format!(
                    "sudo reflector --sort delay -c {country} --save /etc/pacman.d/mirrorlist -p https"
                )
                .as_str()
            ],
        ),"Failed to generate mirrorlist");
            assert!(exec(
            "sh",
            &[
                "-c",
                "sudo sed -i 's/#ParallelDownloads = 5/ParallelDownloads = 5/g' /etc/pacman.conf"
            ]
        ),"Failed to set Parallel download to 5");
            assert!(
                exec("sh", &["-c", "paru -Syyu"]),
                "Failed to update mirrors"
            );
            return self;
        }
        self.configure_mirrors()
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

    ///
    /// # Panics
    ///
    pub fn choose_profile(&mut self) -> &mut Self {
        let profiles: Vec<&str> = MultiSelect::new(
            "Select profiles",
            vec![
                "@gnome",
                "@deepin",
                "@kde",
                "@i3",
                "@xmonad",
                "@admin",
                "@ai",
                "@3d-printing",
                "@containers",
                "@virtualisation",
                "@none",
            ],
        )
        .prompt()
        .unwrap();
        if profiles.is_empty() {
            return self.choose_profile();
        }
        if profiles.get(1).unwrap().eq(&"@none") {
            return self.choose_packages();
        }

        let mut builder = Builder::default();
        for profile in &profiles {
            builder.push_record([profile.to_string()]);
        }
        let table = builder.build().with(Style::modern()).to_string();

        println!("{table}");
        match prompt_confirmation("Install profiles ?") {
            Ok(true) => {
                for profile in &profiles {
                    assert!(install_profile(profile));
                }
                return self.choose_packages();
            }
            Ok(false) | Err(_) => self.choose_profile(),
        }
    }

    ///
    /// # Panics
    ///
    pub fn configure_hostname(&mut self) -> &mut Self {
        assert!(
            exec(
                "sh",
                &["-c", format!("echo {} > hostname", self.hostname).as_str()],
            ),
            "Failed to define hostname"
        );

        assert!(
            exec("sh", &["-c", "sudo install -m 644 hostname /etc/hostname"],),
            "Failed to install hostname"
        );

        assert!(
            exec("sh", &["-c", "rm hostname"]),
            "Failed to remove tmp hostname file"
        );
        self
    }
    ///
    /// # Panics
    ///
    pub fn choose_hostname(&mut self) -> &mut Self {
        self.hostname.clear();
        let hostname = Text::new("Please enter your hostname : ").prompt().unwrap();
        if hostname.is_empty() {
            return self.choose_hostname();
        }
        self.hostname.push_str(hostname.as_str());
        self
    }

    ///
    /// # Panics
    ///
    pub fn confirm(&mut self) -> ExitCode {
        let ok_lang = prompt_confirmation(format!("Use lang : {}", self.lang).as_str()).unwrap();
        if !ok_lang {
            return self.choose_language().confirm();
        }
        let ok_locale =
            prompt_confirmation(format!("Use locales : {:?}", self.locales).as_str()).unwrap();
        if !ok_locale {
            return self.choose_locales().confirm();
        }
        let ok_timezone =
            prompt_confirmation(format!("Use timezone : {}", self.timezone).as_str()).unwrap();
        if !ok_timezone {
            return self.choose_timezone().confirm();
        }
        let ok_keymap =
            prompt_confirmation(format!("Use keymap : {}", self.keymap).as_str()).unwrap();
        if !ok_keymap {
            return self.choose_keymap().confirm();
        }
        let ok_hostname =
            prompt_confirmation(format!("Use hostname : {}", self.hostname).as_str()).unwrap();
        if !ok_hostname {
            return self.choose_hostname().confirm();
        }

        self.run()
    }

    ///
    /// # Panics
    ///
    pub fn upgrade(&mut self) -> ExitCode {
        assert!(
            exec("sh", &["-c", "paru -Syu && flatpak update"]),
            "Failed to update the system"
        );
        self.quit("Updated successfully")
    }

    ///
    /// # Panics
    ///
    pub fn upgrade_and_reboot(&mut self) -> ExitCode {
        assert!(
            exec("sh", &["-c", "paru -Syu && flatpak update"]),
            "Failed to update the system"
        );
        assert!(exec(
            "sh",
            &[
                "-c",
                "sudo shutdown -r +5 \"Save your work! This system will shut down in five minutes\""
            ]
        ),"Failed to program the reboot of your system");
        self.quit("Save your works ! Your computer will reboot after five minutes.")
    }

    ///
    /// # Panics
    ///
    pub fn cancel_reboot(&mut self) -> ExitCode {
        assert!(
            exec("sh", &["-c", "shutdown -c"]),
            "Cancelation of rhe reboot task has failed"
        );
        self.quit("The reboot has been canceled successfully")
    }

    ///
    /// # Panics
    ///
    pub fn check_update(&mut self) -> ExitCode {
        let o = File::create("/tmp/updates").expect("failed to create update");
        let output = Command::new("paru")
            .arg("-Q")
            .arg("-u")
            .stdout(o)
            .current_dir(".")
            .output()
            .unwrap();
        if output.status.success() {
            let up = std::fs::read_to_string("/tmp/updates").expect("failed to parse update file");
            let lines: Vec<String> = up.lines().map(String::from).collect();
            for line in lines {
                println!("{line}");
            }
            return self.quit("Run arch to update");
        }
        self.quit("System is up to date")
    }

    ///
    /// # Panics
    ///
    pub fn man(&mut self) -> &mut Self {
        assert!(
            exec("sh", &["-c", "w3m man.archlinux.org"]),
            "Failed to navigate on website"
        );
        self
    }
    ///
    /// # Panics
    ///
    pub fn aur(&mut self) -> &mut Self {
        assert!(
            exec("sh", &["-c", "w3m aur.archlinux.org"]),
            "Failed to navigate on aur website"
        );
        self
    }
    ///
    /// # Panics
    ///
    pub fn packages(&mut self) -> &mut Self {
        assert!(
            exec("sh", &["-c", "w3m archlinux.org/packages/"]),
            "Failed to navigate on arch website"
        );
        self
    }

    ///
    /// # Panics
    ///
    pub fn download_update(&mut self) -> ExitCode {
        assert!(
            exec("sh", &["-c", "checkupdates -d"]),
            "System is up to date"
        );
        self.quit("Run -> arch --update in order to update your system")
    }

    ///
    /// # Panics
    ///
    pub fn save_user(&mut self, username: &str) -> &mut Self {
        self.user.clear();
        self.user.push_str(username);
        self
    }

    pub fn create_new_user(&mut self) -> &mut Self {
        assert!(exec(
            "sh",
            &[
                "-c",
                "sudo useradd -m -g wheel -c 'arch' -s /usr/bin/bash arch"
            ]
        ));
        assert!(exec("sh", &["-c", "sudo passwd arch"]));
        assert!(exec(
            "sh",
            &["-c", "sudo echo 'arch ALL=(ALL) ALL' > /etc/sudoers.d/arch"]
        ));
        self
    }
    pub fn checkout_new_user(&mut self) -> &mut Self {
        assert!(exec("sh", &["-c", "sudo su - arch"]));
        self
    }

    pub fn remove_user(&mut self) -> &mut Self {
        assert!(exec(
            "sh",
            &[
                "-c",
                format!("sudo userdel --force --remove {}", self.user).as_str()
            ]
        ));
        self
    }

    ///
    /// # Panics
    ///
    pub fn refresh_cache(&mut self) -> &mut Self {
        assert!(exec(
            "sh",
            &["-c", "pacman -Sl core | cut -d ' ' -f 2 > packages"]
        ));
        assert!(exec(
            "sh",
            &["-c", "pacman -Sl extra | cut -d ' ' -f 2 >> packages"]
        ));
        assert!(exec(
            "sh",
            &["-c", "pacman -Sl multilib | cut -d ' ' -f 2 >> packages"]
        ));
        assert!(exec("sh", &["-c", "pacman -Sg >> packages"]));
        assert!(exec(
            "sh",
            &["-c", "paru -Sl aur | cut -d ' ' -f 2 >> packages"]
        ));
        assert!(exec(
            "sh",
            &["-c", "sudo install -m 644 packages /tmp/packages"]
        ));
        assert!(exec("sh", &["-c", "rm packages"]));
        self
    }
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

///
/// # Panics
///
fn install_packages(packages: &[String]) -> i32 {
    for pkg in packages {
        if pkg.contains("arch") || pkg.contains("-S") {
            continue;
        }
        assert!(
            exec("sh", &["-c", format!("paru -S {pkg}").as_str()]),
            "{}",
            format!("Failed to install the {pkg} package").as_str()
        );
    }
    0
}

///
/// # Panics
///
fn remove_packages(packages: &[String]) -> i32 {
    for pkg in packages {
        if pkg.contains("arch") || pkg.contains("-R") {
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
fn create_iso() -> ExitCode {
    exit(0);
}
///
/// # Panics
///
fn install() -> ExitCode {
    let start = Confirm::new("Start installation")
        .with_default(false)
        .prompt()
        .unwrap();
    if start.eq(&false) {
        println!("Bye");
        exit(1);
    }

    let license = Table::new([fs::read_to_string("/usr/share/licenses/arch/LICENSE")
        .expect("No license has been found")
        .as_str()])
    .with(Style::modern())
    .to_string();
    println!("{license}\n",);

    let license = Confirm::new("Accept license ?")
        .with_default(false)
        .prompt()
        .unwrap();
    if start.eq(&false) {
        println!("Bye");
        exit(1);
    }

    if license.eq(&false) {
        println!("Bye");
        exit(1);
    }
    Arch::new()
        .check_network()
        .systemd()
        .news()
        .forums()
        .wiki()
        .configure_mirrors()
        .choose_language()
        .choose_locales()
        .choose_keymap()
        .choose_timezone()
        .choose_hostname()
        .choose_profile()
        .configure_users()
        .confirm()
}

fn reinstall() -> ExitCode {
    Arch::new()
        .check_network()
        .systemd()
        .news()
        .forums()
        .wiki()
        .save_user(
            std::env::var("USER")
                .expect("Failed to fin username")
                .as_str(),
        )
        .create_new_user()
        .checkout_new_user()
        .remove_user()
        .configure_mirrors()
        .choose_language()
        .choose_locales()
        .choose_keymap()
        .choose_timezone()
        .choose_hostname()
        .configure_locale()
        .configure_keymap()
        .configure_hostname()
        .choose_profile()
        .configure_timezone()
        .configure_users()
        .confirm()
}

fn reconfigure() -> ExitCode {
    let profile = std::fs::read_to_string(format!(
        "{}/.config/arch/desktop",
        std::env::var("HOME").expect("Failed to find HOME")
    ))
    .expect("failed to get profile");
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
                format!("xargs -d '\n' -a {profile} paru --noconfirm -Rdd").as_str()
            ]
        ),
        "{}",
        format!("failed to remove {profile}").as_str()
    );
    if profile.eq("@gnome") {
        assert!(
            exec("sh", &["-c", "sudo systemctl disable --now gdm"]),
            "Failed to disable gdm"
        );
    } else if profile.eq("@kde") {
        assert!(
            exec("sh", &["-c", "sudo systemctl disable --now sddm"]),
            "Failed to disable sddm"
        );
    } else if profile.eq("@deepin") || profile.eq("@xmonad") || profile.eq("@i3") {
        assert!(
            exec("sh", &["-c", "sudo systemctl disable --now lightdm"]),
            "Failed to disable lightdm"
        );
        if profile.eq("@xmonad") {
            assert!(
                exec("sh", &["-c", "rm -rf ~/.xmonad "]),
                "Failed to remove xmonad config"
            );
        }
        if profile.eq("@i3") {
            assert!(
                exec("sh", &["-c", "rm -rf ~/.config/i3 "]),
                "Failed to remove i3 config"
            );
        }
    }
    assert!(
        exec("sh", &["-c", format!("rm {profile}").as_str()]),
        "Failed to remove profile config"
    );
    reinstall()
}
fn main() -> ExitCode {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        return Arch::new().upgrade();
    }
    if args.len() > 1 && args.get(1).expect("failed to get argument").eq("-S") {
        exit(install_packages(&args));
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-i") || args.get(1).unwrap().eq("--setup") {
        return install();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-h") || args.get(1).unwrap().eq("--help") {
        let _ = help();
        exit(0);
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--install") {
        return Arch::new()
            .choose_packages()
            .install_package()
            .quit("Packages installed successfully");
    }
    if args.len() == 3 && args.get(1).unwrap().eq("-s") || args.get(1).unwrap().eq("--search") {
        assert!(exec(
            "sh",
            &[
                "-c",
                format!("paru -Ss {} | more", args.get(2).unwrap()).as_str()
            ]
        ));
        exit(0);
    }
    if args.len() >= 2 && args.get(1).expect("failed to get argument").eq("-R") {
        exit(remove_packages(&args));
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-R") || args.get(1).unwrap().eq("--uninstall") {
        return Arch::new()
            .choose_packages()
            .remove_package()
            .quit("Packages uninstalled successfully");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-M") || args.get(1).unwrap().eq("--mirrors") {
        return Arch::new()
            .check_network()
            .configure_mirrors()
            .quit("Mirrors has been updated successfully");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-C") || args.get(1).unwrap().eq("--check") {
        return Arch::new().check_update();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-u") || args.get(1).unwrap().eq("--update") {
        return Arch::new().upgrade();
    }
    if args.len() == 3 && args.get(1).unwrap().eq("--update") && args.get(2).unwrap().eq("-r") {
        return Arch::new().upgrade_and_reboot();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--upgrade") || args.get(1).unwrap().eq("-u") {
        return Arch::new().upgrade_and_reboot();
    }
    if args.len() == 3 && args.get(1).unwrap().eq("-r") && args.get(2).unwrap().eq("--update") {
        return Arch::new().upgrade_and_reboot();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-U") || args.get(1).unwrap().eq("--upgrade") {
        return Arch::new().upgrade_and_reboot();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-w") || args.get(1).unwrap().eq("--wiki") {
        return Arch::new().wiki().quit("Wiki exit success");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-m")
        || args.get(1).unwrap().eq("--man")
        || args.get(1).unwrap().eq("--woman")
    {
        return Arch::new().man().quit("Man exit success");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-f") || args.get(1).unwrap().eq("--forum") {
        return Arch::new().forums().quit("Forum exit successfully");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-a") || args.get(1).unwrap().eq("--aur") {
        return Arch::new().aur().quit("Aur exit successfully");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-n") || args.get(1).unwrap().eq("--news") {
        return Arch::new().news().quit("News exit successfully");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-c") || args.get(1).unwrap().eq("--cache") {
        return Arch::new().refresh_cache().quit("Cache udate successfully");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("-d") || args.get(1).unwrap().eq("--deps") {
        return Arch::new()
            .choose_packages()
            .install_dependencies()
            .quit("Dependencies has been installed successfully");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--download-updates") {
        return Arch::new().download_update();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--cancel") || args.get(1).unwrap().eq("-x") {
        return Arch::new().cancel_reboot();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--version") || args.get(1).unwrap().eq("-v") {
        println!("arch version : {VERSION}");
        exit(0);
    }

    if args.len() == 2 && args.get(1).unwrap().eq("--setup-new-config") {
        return reconfigure();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--create-iso") {
        return create_iso();
    }
    exit(help());
}
