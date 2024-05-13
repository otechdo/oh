use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use crate::conf::System::Uefi;
use crate::conf::{System, BOOTLOADER, COUNTRIES, KEYMAPS, LOCALES, PROFILES, TIMEZONES, KEYMAP_LAYOUTS, KEYMAP_MODELS, KEYMAP_OPTIONS};
use inquire::{Confirm, MultiSelect, Select, Text};
use std::time::Instant;
use crate::desktop::{BUDGIE, CINNAMON, DEEPIN, GNOME, KDE, LXQT};
use crate::diy::DIY;
use crate::server::ADMIN;
use crate::window::{AWESOME, BSPWM, HYPRLAND, I3, QTILE, XMONAD};

pub struct Arch {
    pub locales: Vec<String>,
    pub services: Vec<String>,
    pub packages: Vec<String>,
    pub profiles: HashMap<String, Vec<String>>,
    pub timezone: String,
    pub keymap: String,
    pub hostname: String,
    pub boot: String,
    pub begin: Instant,
    pub system: System,
    mirror: String,
    keymap_layout: String,
    keymap_model: String,
    keymap_options: String,
}

fn profiles(profiles: &[String]) -> HashMap<String, Vec<String>>
{
    let mut h: HashMap<String, Vec<String>> = HashMap::new();

    for p in profiles {
        match p.as_str() {
            "none" => assert!(h.insert(p.to_string(), Vec::new()).is_none()),
            "gnome" => assert!(h.insert(p.to_string(), GNOME.map(|x| (*x).to_string()).to_vec()).is_none()),
            "kde" => assert!(h.insert(p.to_string(), KDE.map(|x| (*x).to_string()).to_vec()).is_none()),
            "deepin" => assert!(h.insert(p.to_string(), DEEPIN.map(|x| (*x).to_string()).to_vec()).is_none()),
            "budgie" => assert!(h.insert(p.to_string(), BUDGIE.map(|x| (*x).to_string()).to_vec()).is_none()),
            "cinnamon" => assert!(h.insert(p.to_string(), CINNAMON.map(|x| (*x).to_string()).to_vec()).is_none()),
            "lxqt" => assert!(h.insert(p.to_string(), LXQT.map(|x| (*x).to_string()).to_vec()).is_none()),
            "i3" => assert!(h.insert(p.to_string(), I3.map(|x| (*x).to_string()).to_vec()).is_none()),
            "xmonad" => assert!(h.insert(p.to_string(), XMONAD.map(|x| (*x).to_string()).to_vec()).is_none()),
            "bspwm" => assert!(h.insert(p.to_string(), BSPWM.map(|x| (*x).to_string()).to_vec()).is_none()),
            "cockpit" => assert!(h.insert(p.to_string(), ADMIN.map(|x| (*x).to_string()).to_vec()).is_none()),
            "3d-printing" => assert!(h.insert(p.to_string(), DIY.map(|x| (*x).to_string()).to_vec()).is_none()),
            "qtile" => assert!(h.insert(p.to_string(), QTILE.map(|x| (*x).to_string()).to_vec()).is_none()),
            "awesome" => assert!(h.insert(p.to_string(), AWESOME.map(|x| (*x).to_string()).to_vec()).is_none()),
            "hyprland" => assert!(h.insert(p.to_string(), HYPRLAND.map(|x| (*x).to_string()).to_vec()).is_none()),
            &_ => todo!()
        }
    }
    h
}

impl Default for Arch {
    fn default() -> Self {
        Self {
            locales: Vec::new(),
            services: Vec::new(),
            packages: Vec::new(),
            profiles: HashMap::new(),
            timezone: String::new(),
            keymap: String::new(),
            hostname: String::new(),
            boot: String::new(),
            mirror: String::new(),
            keymap_layout: String::new(),
            keymap_model: String::new(),
            begin: Instant::now(),
            system: Uefi,
            keymap_options: String::new(),
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

    ///
    /// Choose the hostname
    ///
    fn choose_hostname(&mut self) -> &mut Self;
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

    fn choose_hostname(&mut self) -> &mut Self {
        loop {
            self.hostname.clear();
            self.hostname.push_str(Text::new("Enter the hostname : ").prompt().unwrap().as_str());
            if !self.hostname.is_empty() {
                break;
            }
        }
        self
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
            if !self.mirror.is_empty() && Confirm::new(format!("Use {} country for mirror list ? ", self.mirror).as_str())
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&true)
            {
                break;
            }
        }
        self
    }

    fn choose_locales(&mut self) -> &mut Self {
        let mut locales: Vec<String> = Vec::new();
        loop {
            self.locales.clear();
            locales.clear();
            let lo = MultiSelect::new("Choose locales : ", LOCALES.to_vec())
                .prompt()
                .unwrap();
            for &l in &lo {
                locales.push(l.to_string());
            }
            if !locales.is_empty() && Confirm::new(
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
        self
    }
    fn choose_profiles(&mut self) -> &mut Self {
        loop {
            let mut wishes: Vec<String> = Vec::new();
            self.profiles.clear();
            let pp = MultiSelect::new("Choose profiles", PROFILES.to_vec())
                .prompt().unwrap();
            for &p in &pp {
                wishes.push(p.to_string());
            }
            if !pp.is_empty() && Confirm::new(format!("Install {wishes:?} ?").as_str())
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&true)
            {
                self.profiles = profiles(&wishes);
                break;
            }
        }
        self
    }

    fn choose_keymap(&mut self) -> &mut Self {
        loop {
            self.keymap.clear();
            self.keymap_layout.clear();
            self.keymap_model.clear();
            self.keymap_options.clear();

            self.keymap.push_str(
                Select::new("Select a keymap", KEYMAPS.to_vec())
                    .prompt()
                    .unwrap(),
            );
            if self.keymap.is_empty() {
                continue;
            }
            if Confirm::new(format!("Use {} keymap ?", self.keymap).as_str())
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&false)
            {
                continue;
            }
            self.keymap_layout.push_str(
                Select::new("Select a keymap layout", KEYMAP_LAYOUTS.to_vec())
                    .prompt()
                    .unwrap(),
            );
            if self.keymap_layout.is_empty() { continue; }

            if Confirm::new(format!("Use {} keymap layout ?", self.keymap_layout).as_str())
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&false)
            {
                continue;
            }
            self.keymap_model.push_str(
                Select::new("Select a keymap model", KEYMAP_MODELS.to_vec())
                    .prompt()
                    .unwrap(),
            );
            if self.keymap_model.is_empty() { continue; }

            if Confirm::new(format!("Use {} keymap model ?", self.keymap_model).as_str())
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&false)
            {
                continue;
            }

            self.keymap_options.push_str(
                Select::new("Select a keymap options", KEYMAP_OPTIONS.to_vec())
                    .prompt()
                    .unwrap(),
            );
            if self.keymap_options.is_empty() { continue; }
            if Confirm::new(format!("Use {} keymap options ?", self.keymap_options).as_str())
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&false)
            {
                continue;
            }
            break;
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
            if !self.boot.is_empty() && Confirm::new(format!("Use {} bootloader ?", self.boot).as_str())
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&true)
            {
                break;
            }
        }
        self
    }

    fn configure_keymap(&mut self) -> &mut Self {
        let mut keymap = File::create("/etc/vconsole.conf").expect("failed to create vconsole.conf");
        assert!(keymap.write(format!("KEYMAP={}\nXKBLAYOUT={}\nXKBMODEL={}\nXKBOPTIONS={}", self.keymap, self.keymap_layout, self.keymap_model, self.keymap_options).as_bytes()).is_ok());
        assert!(keymap.sync_all().is_ok());
        self
    }

    fn configure_locales(&mut self) -> &mut Self {
        todo!()
    }

    fn configure_hostname(&mut self) -> &mut Self {
        let mut hostname = File::create("/etc/hostname").expect("failed to create hostname");
        assert!(hostname.write(self.hostname.as_bytes()).is_ok());
        assert!(hostname.sync_all().is_ok());
        self
    }

    fn configure_profiles(&mut self) -> &mut Self {
        for (profile, packages) in &self.profiles {
            println!("Installing {profile} profile");
            assert!(Command::new("paru").arg("-S").arg("--noconfirm").args(packages).spawn().expect("paru not founded").wait().expect("").success());
        }
        self
    }
}
