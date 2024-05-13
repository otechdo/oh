use crate::conf::System;
use crate::conf::System::Uefi;
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
    fn choose_mirrors(&mut self) -> Self;

    ///
    /// Choose system locales
    ///
    fn choose_locales(&mut self) -> Self;

    ///
    /// Choose profiles
    ///
    fn choose_profiles(&mut self) -> Self;

    ///
    /// Choose system keymaps
    ///
    fn choose_keymap(&mut self) -> Self;

    ///
    /// Choose bootloader
    ///
    fn choose_bootloader(&mut self) -> Self;

    ///
    /// Configure the keymap
    ///
    fn configure_keymap(&mut self) -> Self;

    ///
    /// Configure the locales
    ///
    fn configure_locales(&mut self) -> Self;

    ///
    /// Configure the hostname
    ///
    fn configure_hostname(&mut self) -> Self;

    ///
    /// Configure the profiles
    ///
    fn configure_profiles(&mut self) -> Self;
}
