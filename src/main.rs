#![allow(clippy::multiple_crate_versions)]

use inquire::{prompt_confirmation, Confirm, MultiSelect, Password, Select, Text};
use regex::Regex;
use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::{read_to_string, BufRead};
use std::path::Path;
use std::process::{exit, Command, ExitCode};
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
    profile: String,
    lang: String,
    packages: Vec<String>,
    root: HashMap<bool, String>,
    users: Vec<Users>,
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
            profile: String::new(),
            hostname: String::new(),
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
                exec("sh", &["-c", format!("yay -S --noconfirm {pkg}").as_str()]),
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
                exec(
                    "sh",
                    &["-c", format!("yay -S {pkg} --noconfirm --asdeps").as_str()]
                ),
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
                exec("sh", &["-c", format!("yay -Rns {pkg}").as_str()]),
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
        0
    }

    ///
    /// if failed to get locale
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
            self.lang = locale.to_string();

            self
        }
    }

    ///
    /// if failed to get locale
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
        if locales.is_empty() {
            self.choose_locales()
        } else {
            for l in locale {
                self.locales.push(l.to_string());
            }

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
        let keymap = Text::new("Please enter your keymap : ").prompt().unwrap();
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

        for locale in &self.locales {
            assert!(exec(
                "sh",
                &[
                    "-c",
                    format!(
                        "sudo sed -i 's/#{} UTF-8/{} UTF-8/g' /etc/locale.gen",
                        locale, locale
                    )
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
        if Path::new("/tmp/pkgs").exists() {
            self.packages.clear();
            assert!(self.packages.is_empty());
            loop {
                let p = MultiSelect::new("Select packages : ", parse_file_lines("/tmp/pkgs"))
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
            if user.sudoers {
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
            } else {
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
            return self
                .install_profile()
                .install_package()
                .create_users()
                .configure_timezone()
                .configure_locale()
                .configure_keymap()
                .configure_hostname()
                .systemd()
                .quit_installer();
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
        let zone = Text::new("Please enter your timezone : ").prompt().unwrap();
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
        let country = Text::new("Please enter your country ? : ")
            .prompt()
            .unwrap();

        if country.is_empty() {
            return self.configure_mirrors();
        }
        let confirm_mirror = prompt_confirmation(
            format!("Set your mirrorlist to the {country} country : ").as_str(),
        );
        match confirm_mirror {
            Ok(false) | Err(_) => self.configure_mirrors(),
            Ok(true) => {
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
                assert!(exec("sh", &["-c", "yay -Syyu"]), "Failed to update mirrors");
                self
            }
        }
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
        let profile = Select::new(
            "Select a profile",
            vec!["@gnome", "@deepin", "@kde", "@i3", "@xmonad", "@none"],
        )
        .prompt()
        .unwrap();
        if profile.is_empty() {
            return self.choose_profile();
        }
        self.profile = profile.to_string();

        self.choose_packages()
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
        let ok_profile =
            prompt_confirmation(format!("Use profile : {}", self.profile).as_str()).unwrap();
        if !ok_profile {
            return self.choose_profile().confirm();
        }
        self.run()
    }

    ///
    /// # Panics
    ///
    pub fn upgrade(&mut self) -> ExitCode {
        assert!(
            exec("sh", &["-c", "yay -Syu && flatpak update"]),
            "Failed to update the system"
        );
        self.quit("Updated successfully")
    }

    ///
    /// # Panics
    ///
    pub fn upgrade_and_reboot(&mut self) -> ExitCode {
        assert!(
            exec("sh", &["-c", "yay -Syu && flatpak update"]),
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
        assert!(exec("sh", &["-c", "checkupdates"]), "System is up to date");
        self.quit("Run -> arch --update in order to update your system")
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
    fn install_profile(&mut self) -> &mut Self {
        println!("{}", format!("using {}", self.profile).as_str());
        assert!(Command::new("wget")
            .arg("-q")
            .arg(
                format!(
                    "https://raw.githubusercontent.com/otechdo/arch/main/arch/profiles/{}",
                    self.profile
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
                    format!(
                        "xargs -d '\n' -a {} yay --noconfirm --needed -Syu",
                        self.profile
                    )
                    .as_str()
                ]
            ),
            "{}",
            format!("failed to install {}", self.profile).as_str()
        );
        if self.profile.eq("@gnome") {
            assert!(
                exec("sh", &["-c", "sudo systemctl enable gdm"]),
                "Failed to enable gdm"
            );
        } else if self.profile.eq("@kde") {
            assert!(
                exec("sh", &["-c", "sudo systemctl enable sddm"]),
                "Failed to enable sddm"
            );
        } else if self.profile.eq("@deepin") || self.profile.eq("@xmonad") || self.profile.eq("@i3")
        {
            assert!(
                exec("sh", &["-c", "sudo systemctl enable lightdm"]),
                "Failed to enable lightdm"
            );
            if self.profile.eq("@xmonad") {
                assert!(
                            exec("sh", &["-c", "mkdir ~/.xmonad && wget -q https://raw.githubusercontent.com/otechdo/arch/main/arch/config/xmonad/xmonad.hs && mv xmonad.hs ~/.xmonad && touch ~/.xmonad/build && chmod +x ~/.xmonad/build && xmonad --recompile"]),
                            "Failed to configure xmonad"
                            );
            }
        }
        std::fs::remove_file(self.profile.clone()).expect("failed to profile file");
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
    pub fn refresh_cache(&mut self) -> ExitCode {
        assert!(exec(
            "sh",
            &["-c", "pacman -Sl core | cut -d ' ' -f 2 > pkgs"]
        ));
        assert!(exec(
            "sh",
            &["-c", "pacman -Sl extra | cut -d ' ' -f 2 >> pkgs"]
        ));
        assert!(exec(
            "sh",
            &["-c", "pacman -Sl multilib | cut -d ' ' -f 2 >> pkgs"]
        ));
        assert!(exec("sh", &["-c", "pacman -Sg >> pkgs"]));
        assert!(exec("sh", &["-c", "yay -Sl aur | cut -d ' ' -f 2 >> pkgs"]));
        assert!(exec("sh", &["-c", "install -m 644 pkgs /tmp/pkgs"]));
        assert!(exec("sh", &["-c", "rm pkgs"]));
        self.quit("Packages cache updated successfully")
    }
}

fn help() -> i32 {
    println!("arch setup                    : Configure a new arch\narch --help                   : Display help\narch --install-packages       : Install packages as inplicit\narch --install-dependencies   : Install packages as dependencies\narch --remove-packages        : Remove selected packages\narch --update-mirrors         : Update arch mirrors\narch --update                 : Update arch\narch --update-and-reboot      : Update arch and reboot after five minutes\narch --download-updates       : Download all updates\narch --check-updates          : Check and print all available updates\narch --cancel-reboot          : Cancel rebooting after five minutes");
    1
}

///
/// # Panics
///
fn install_packages(pkgs: &[String]) -> i32 {
    for pkg in pkgs {
        if pkg.contains("arch") || pkg.contains("-S") {
            continue;
        }
        assert!(
            exec("sh", &["-c", format!("yay -S --noconfirm {pkg}").as_str()]),
            "{}",
            format!("Failed to install the {pkg} package").as_str()
        );
        assert!(notifme::Notification::new()
            .app("arch")
            .summary(format!("{pkg} Installed").as_str())
            .body(format!("{pkg} has been installed successfully").as_str())
            .timeout(5)
            .send());
    }
    0
}

///
/// # Panics
///
fn remove_packages(pkgs: &[String]) -> i32 {
    for pkg in pkgs {
        if pkg.contains("arch") || pkg.contains("-R") {
            continue;
        }
        assert!(
            exec("sh", &["-c", format!("yay -Rns {pkg}").as_str()]),
            "{}",
            format!("Failed to install the {pkg} package").as_str()
        );
    }
    0
}

///
/// # Panics
///
fn install() -> ExitCode {
    Arch::new()
        .check_network()
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
fn main() -> ExitCode {
    let args: Vec<String> = args().collect();
    if args.len() >= 2 && args.get(1).expect("failed to get argument").eq("-S") {
        exit(install_packages(&args));
    }
    if args.len() >= 2 && args.get(1).expect("failed to get argument").eq("-R") {
        exit(remove_packages(&args));
    }

    if args.len() == 2 && args.get(1).expect("failed to get argument").eq("-a") {
        return Arch::new().aur().quit("Exit aur successfully");
    }

    if args.len() == 2 && args.get(1).expect("failed to get argument").eq("--man")
        || args.get(1).expect("failed to get argument").eq("-m")
    {
        return Arch::new().man().quit("Exit man successfully");
    }

    if args.len() == 2 && args.get(1).unwrap().eq("setup") {
        return install();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--news") || args.get(1).unwrap().eq("-n") {
        return Arch::new().news().quit("News exit success");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--wiki") || args.get(1).unwrap().eq("-w") {
        return Arch::new().wiki().quit("Wiki exit success");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--man") || args.get(1).unwrap().eq("-m") {
        return Arch::new().man().quit("Man exit success");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--forums") || args.get(1).unwrap().eq("-f") {
        return Arch::new().forums().quit("Forums exit success");
    }
    if args.len() == 2 && args.get(1).expect("failed to get argument").eq("-a")
        || args.get(1).unwrap().eq("--aur")
    {
        return Arch::new().aur().quit("Exit aur successfully");
    }

    if args.len() == 2 && args.get(1).unwrap().eq("--install") || args.get(1).unwrap().eq("-i") {
        return Arch::new()
            .choose_packages()
            .install_package()
            .quit("Packages installed success");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--install-dependencies") {
        return Arch::new()
            .choose_packages()
            .install_dependencies()
            .quit("Dependencies as been installed successfully");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--remove") || args.get(1).unwrap().eq("-R") {
        return Arch::new()
            .choose_packages()
            .remove_package()
            .quit("Packages has been removed successfully");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--update-mirrors") {
        return Arch::new()
            .check_network()
            .configure_mirrors()
            .quit("Mirrors has been updated successfully");
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--help") {
        let _ = help();
        exit(0);
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--update") {
        return Arch::new().upgrade();
    }

    if args.len() == 2 && args.get(1).unwrap().eq("--refresh-cache") {
        return Arch::new().refresh_cache();
    }
    if args.len() == 3 && args.get(1).unwrap().eq("--update") && args.get(2).unwrap().eq("-r") {
        return Arch::new().upgrade_and_reboot();
    }
    if args.len() == 3 && args.get(1).unwrap().eq("-r") && args.get(2).unwrap().eq("--update") {
        return Arch::new().upgrade_and_reboot();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--update-and-reboot") {
        return Arch::new().upgrade_and_reboot();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--check-updates") {
        return Arch::new().check_update();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--download-updates") {
        return Arch::new().download_update();
    }
    if args.len() == 2 && args.get(1).unwrap().eq("--cancel-reboot") {
        return Arch::new().cancel_reboot();
    }
    exit(help());
}
