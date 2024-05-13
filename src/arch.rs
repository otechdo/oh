use crate::conf::System::Uefi;
use crate::conf::{System, BOOTLOADER, COUNTRIES, KEYMAPS, LOCALES, PROFILES, TIMEZONES};
use inquire::{Confirm, MultiSelect, Select};
use std::time::Instant;

pub struct Arch {
    pub locales: Vec<String>,
    pub services: Vec<String>,
    pub packages: Vec<String>,
    pub timezone: String,
    pub keymap: String,
    pub hostname: String,
    pub boot: String,
    pub begin: Instant,
    pub system: System,
    mirror: String,
}

impl Default for Arch {
    fn default() -> Self {
        Self {
            locales: Vec::new(),
            services: Vec::new(),
            packages: Vec::new(),
            timezone: String::new(),
            keymap: String::new(),
            hostname: String::new(),
            boot: String::new(),
            mirror: String::new(),
            begin: Instant::now(),
            system: Uefi,
        }
    }
}

pub trait Os {
    ///
    /// List al available languages
    ///
    fn list_language(&mut self) -> Vec<&str>;

    ///
    /// List al available keymaps
    ///
    fn list_keymaps(&mut self) -> Vec<&str>;

    ///
    /// List all available bootloader
    ///
    fn list_bootloader(&mut self) -> Vec<&str>;

    ///
    /// List all available timezones
    ///
    fn list_timezones(&mut self) -> Vec<&str>;
}

pub trait Installer {
    ///
    /// Choose system mirror
    ///
    fn choose_mirrors(&mut self) -> &mut Self;

    ///
    /// Choose system locales
    ///
    fn choose_locales(&mut self) -> &mut Self;

    ///
    /// Choose profiles
    ///
    fn choose_profiles(&mut self) -> &mut Self;

    ///
    /// Choose system keymaps
    ///
    fn choose_keymap(&mut self) -> &mut Self;

    ///
    /// Choose bootloader
    ///
    fn choose_bootloader(&mut self) -> &mut Self;

    ///
    /// Configure the keymap
    ///
    fn configure_keymap(&mut self) -> &mut Self;

    ///
    /// Configure the locales
    ///
    fn configure_locales(&mut self) -> &mut Self;

    ///
    /// Configure the hostname
    ///
    fn configure_hostname(&mut self) -> &mut Self;

    ///
    /// Configure the profiles
    ///
    fn configure_profiles(&mut self) -> &mut Self;
}

impl Os for Arch {
    fn list_language(&mut self) -> Vec<&str> {
        LOCALES.to_vec()
    }

    fn list_keymaps(&mut self) -> Vec<&str> {
        KEYMAPS.to_vec()
    }

    fn list_bootloader(&mut self) -> Vec<&str> {
        BOOTLOADER.to_vec()
    }

    fn list_timezones(&mut self) -> Vec<&str> {
        TIMEZONES.to_vec()
    }
}

impl Installer for Arch {
    fn choose_mirrors(&mut self) -> &mut Self {
        loop {
            self.mirror.clear();
            self.mirror.push_str(
                Select::new("Mirror country", COUNTRIES.to_vec())
                    .prompt()
                    .unwrap(),
            );
            if !self.mirror.is_empty() {
                if Confirm::new(format!("Use {} country for mirror list ? ", self.mirror).as_str())
                    .with_default(false)
                    .prompt()
                    .unwrap()
                    .eq(&true)
                {
                    break;
                }
            }
        }
        self
    }

    fn choose_locales(&mut self) -> &mut Self {
        let mut locales: Vec<String> = Vec::new();
        loop {
            self.locales.clear();
            locales.clear();
            MultiSelect::new("", LOCALES.to_vec())
                .prompt()
                .unwrap()
                .iter()
                .for_each(|x| locales.push(x.to_string()));
            if !locales.is_empty() {
                if Confirm::new(
                    format!(
                        "Use LANG={} LOCALES={locales:?} ",
                        locales.first().expect("failed to get first locale")
                    )
                        .as_str(),
                )
                    .with_default(false)
                    .prompt()
                    .unwrap()
                    .eq(&true)
                {
                    break;
                }
            }
        }
        self
    }
    fn choose_profiles(&mut self) -> &mut Self {
        let mut profiles: Vec<String> = Vec::new();
        loop {
            self.locales.clear();
            profiles.clear();
            MultiSelect::new("Choose profiles", PROFILES.to_vec())
                .prompt()
                .unwrap()
                .iter()
                .for_each(|x| profiles.push(x.to_string()));
            if !profiles.is_empty() {
                if Confirm::new(format!("Install {profiles:?} ?").as_str())
                    .with_default(false)
                    .prompt()
                    .unwrap()
                    .eq(&true)
                {
                    break;
                }
            }
        }
        self
    }

    fn choose_keymap(&mut self) -> &mut Self {
        loop {
            self.keymap.clear();
            self.keymap.push_str(
                Select::new("Select a keymap", KEYMAPS.to_vec())
                    .prompt()
                    .unwrap(),
            );
            if !self.keymap.is_empty() {
                if Confirm::new(format!("Use {} keymap ?", self.keymap).as_str())
                    .with_default(false)
                    .prompt()
                    .unwrap()
                    .eq(&true)
                {
                    break;
                }
            }
        }
        self
    }

    fn choose_bootloader(&mut self) -> &mut Self {
        loop {
            self.boot.clear();
            self.boot.push_str(
                Select::new("Select a bootloader", BOOTLOADER.to_vec())
                    .prompt()
                    .unwrap(),
            );
            if !self.boot.is_empty() {
                if Confirm::new(format!("Use {} bootloader ?", self.boot).as_str())
                    .with_default(false)
                    .prompt()
                    .unwrap()
                    .eq(&true)
                {
                    break;
                }
            }
        }
        self
    }

    fn configure_keymap(&mut self) -> &mut Self {
        todo!()
    }

    fn configure_locales(&mut self) -> &mut Self {
        todo!()
    }

    fn configure_hostname(&mut self) -> &mut Self {
        todo!()
    }

    fn configure_profiles(&mut self) -> &mut Self {
        todo!()
    }
}
