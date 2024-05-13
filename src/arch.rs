use crate::boot::{DISPLAY_MANAGER, LOADER};
use crate::conf::{
    CINNAMON_DESKTOP, COUNTRIES, CUTEFISH_DESKTOP, GNOME_DESKTOP, I3_WINDOW_MANAGER, KDE_DESKTOP,
    KEYMAPS, KEYMAP_LAYOUTS, KEYMAP_MODELS, KEYMAP_OPTIONS, LOCALES, PRINTING, PROFILES,
    QTILE_WINDOW_MANAGER, TIMEZONES, XFCE_DESKTOP,
};
use crate::desktop::{BUDGIE, CINNAMON, CUTEFISH, DEEPIN, GNOME, KDE, LXQT, XFCE};
use crate::diy::DIY;
use crate::hack::HACK;
use crate::programming::{ASSEMBLY, C, D, GO, PHP, PYTHON, R, RUST};
use crate::server::{ADMIN, SSH};
use crate::window::{AWESOME, BSPWM, HYPRLAND, I3, QTILE, XMONAD};
use inquire::{Confirm, MultiSelect, Select, Text};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::time::Instant;

pub struct Arch {
    pub locales: Vec<String>,
    pub services: Vec<String>,
    pub packages: Vec<String>,
    pub profiles: Vec<String>,
    pub timezone: String,
    pub keymap: String,
    pub hostname: String,
    pub boot: String,
    pub begin: Instant,
    pub system: String,
    pub keymap_layout: String,
    pub keymap_model: String,
    pub keymap_options: String,
    pub mirror_country: String,
    pub mirror_sort: String,
    pub mirror_protocol: String,
    pub display_manager: String,
}

impl Default for Arch {
    fn default() -> Self {
        Self {
            locales: Vec::new(),
            services: Vec::new(),
            packages: Vec::new(),
            profiles: Vec::new(),
            timezone: String::new(),
            keymap: String::new(),
            hostname: String::new(),
            boot: String::new(),
            mirror_sort: String::new(),
            mirror_protocol: String::new(),
            mirror_country: String::new(),
            keymap_layout: String::new(),
            keymap_model: String::new(),
            begin: Instant::now(),
            system: String::new(),
            keymap_options: String::new(),
            display_manager: String::new(),
        }
    }
}

pub trait Diy {
    fn install_diy(&mut self) -> &mut Self;
}

pub trait Hacking {
    fn install_hack(&mut self) -> &mut Self;
    fn install_openssh(&mut self) -> &mut Self;
    fn install_printing(&mut self) -> &mut Self;
}

pub trait Server {
    fn install_cockpit(&mut self) -> &mut Self;
}

pub trait Languages {
    fn install_php(&mut self) -> &mut Self;
    fn install_c(&mut self) -> &mut Self;
    fn install_d(&mut self) -> &mut Self;
    fn install_r(&mut self) -> &mut Self;
    fn install_rust(&mut self) -> &mut Self;
    fn install_go(&mut self) -> &mut Self;
    fn install_python(&mut self) -> &mut Self;
    fn install_assembly(&mut self) -> &mut Self;
}

pub trait Desktop {
    ///
    /// Install gnome
    ///
    fn install_gnome(&mut self) -> &mut Self;
    fn install_cutefish(&mut self) -> &mut Self;

    ///
    /// Install deepin
    ///
    fn install_deepin(&mut self) -> &mut Self;

    fn install(&mut self, p: &[&str], display_manager: &str) -> &mut Self;

    ///
    /// Install kde
    ///
    fn install_kde(&mut self) -> &mut Self;
    ///
    /// Install cinnamon
    ///
    fn install_cinnamon(&mut self) -> &mut Self;

    ///
    /// Install budgie
    ///
    fn install_budgie(&mut self) -> &mut Self;

    ///
    /// Install xfce
    ///
    fn install_xfce(&mut self) -> &mut Self;

    ///
    /// Install lxqt
    ///
    fn install_lxqt(&mut self) -> &mut Self;
}

pub trait WindowManager {
    ///
    /// Install i3
    ///
    fn install_i3(&mut self) -> &mut Self;
    ///
    /// Install awesome
    ///
    fn install_awesome(&mut self) -> &mut Self;

    ///
    /// Install xmonad
    ///
    fn install_xmonad(&mut self) -> &mut Self;

    ///
    /// Install bspwm
    ///
    fn install_bspwm(&mut self) -> &mut Self;

    ///
    /// Install qtile
    ///
    fn install_qtile(&mut self) -> &mut Self;

    ///
    /// Install hyprland
    ///
    fn install_hyprland(&mut self) -> &mut Self;
}

pub trait Installer {
    ///
    /// Choose system mirror
    ///
    fn choose_mirrors(&mut self) -> &mut Self;
    ///
    /// Choose the system timezone
    ///
    fn choose_timezone(&mut self) -> &mut Self;

    ///
    /// Choose a hostname
    ///
    fn choose_hostname(&mut self) -> &mut Self;
    ///
    /// Enabled all required services
    ///
    fn enable_services(&mut self) -> &mut Self;

    ///
    /// Run setup
    ///
    fn setup(&mut self) -> i32;

    ///
    /// Quit installer
    ///
    fn quit(&mut self, message: &str) -> i32;

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
    /// Choose display manager
    ///
    fn choose_display_manager(&mut self) -> &mut Self;

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
    /// Configure pacman
    ///
    fn configure_mirrors(&mut self) -> &mut Self;

    ///
    /// Configure timezone
    ///
    fn configure_timezone(&mut self) -> &mut Self;

    ///
    /// Configure display manager
    ///
    fn configure_display_manager(&mut self) -> &mut Self;

    ///
    /// Configure the profiles
    ///
    fn configure_profiles(&mut self) -> &mut Self;
    fn install_profile(&mut self, p: &str);
}

impl Installer for Arch {
    fn choose_mirrors(&mut self) -> &mut Self {
        loop {
            self.mirror_country.clear();
            self.mirror_sort.clear();
            self.mirror_protocol.clear();

            self.mirror_country.push_str(
                Select::new("Mirror country", COUNTRIES.to_vec())
                    .prompt()
                    .unwrap(),
            );
            if self.mirror_country.is_empty()
                || Confirm::new(
                format!("Use {} country for mirror list ? ", self.mirror_country).as_str(),
            )
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&false)
            {
                continue;
            }
            self.mirror_sort.push_str(
                Select::new(
                    "Mirror sort",
                    vec!["delay", "rate", "age", "country", "score"],
                )
                    .prompt()
                    .unwrap(),
            );
            if self.mirror_sort.is_empty()
                || Confirm::new(
                format!("Use {} country for mirror list ? ", self.mirror_sort).as_str(),
            )
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&false)
            {
                continue;
            }

            self.mirror_protocol.push_str(
                Select::new("Mirror sort", vec!["http", "https", "rsync", "ftp"])
                    .prompt()
                    .unwrap(),
            );
            if self.mirror_protocol.is_empty()
                || Confirm::new(
                format!("Use {} country for mirror list ? ", self.mirror_protocol).as_str(),
            )
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

    fn choose_timezone(&mut self) -> &mut Self {
        loop {
            self.timezone.clear();
            self.timezone.push_str(
                Select::new("Select a timezone", TIMEZONES.to_vec())
                    .prompt()
                    .unwrap(),
            );
            if self.timezone.is_empty()
                || Confirm::new(format!("Use {} timezone", self.timezone).as_str())
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

    fn choose_hostname(&mut self) -> &mut Self {
        loop {
            self.hostname.clear();
            self.hostname.push_str(
                Text::new("Enter the hostname : ")
                    .prompt()
                    .unwrap()
                    .as_str(),
            );
            if self.hostname.is_empty()
                || Confirm::new(format!("Use {} hostname", self.hostname).as_str())
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

    fn enable_services(&mut self) -> &mut Self {
        assert!(Command::new("systemctl")
            .arg("enable")
            .arg("NetworkManager.service")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }

    fn setup(&mut self) -> i32 {
        self.choose_mirrors()
            .choose_locales()
            .choose_timezone()
            .choose_keymap()
            .choose_hostname()
            .choose_bootloader()
            .choose_display_manager()
            .choose_profiles()
            .configure_mirrors()
            .configure_locales()
            .configure_keymap()
            .configure_hostname()
            .configure_profiles()
            .configure_timezone()
            .configure_display_manager()
            .enable_services()
            .quit("System is ready to use reboot now")
    }

    fn quit(&mut self, message: &str) -> i32 {
        println!("{message}");
        0
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
            if !locales.is_empty()
                && Confirm::new(
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
            self.profiles.clear();
            let profiles = MultiSelect::new("Choose profiles", PROFILES.to_vec())
                .prompt()
                .unwrap();
            let mut wishes: Vec<String> = Vec::new();
            for &profile in profiles.iter() {
                wishes.push(profile.to_string());
            }
            if self.profiles.is_empty()
                || Confirm::new(format!("Install {wishes:?} ?").as_str())
                .with_default(false)
                .prompt()
                .unwrap()
                .eq(&false)
            {
                return self.choose_profiles();
            }
            self.profiles = wishes;
            break;
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
            if self.keymap_layout.is_empty() {
                continue;
            }

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
            if self.keymap_model.is_empty() {
                continue;
            }
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
            if self.keymap_options.is_empty() {
                continue;
            }
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
                Select::new("Select a bootloader", LOADER.to_vec())
                    .prompt()
                    .unwrap(),
            );
            if !self.boot.is_empty()
                && Confirm::new(format!("Use {} bootloader ?", self.boot).as_str())
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

    fn choose_display_manager(&mut self) -> &mut Self {
        loop {
            self.display_manager.clear();
            self.display_manager.push_str(
                Select::new(
                    "Select a display manager",
                    vec!["none", "gdm", "lightdm", "sddm"],
                )
                    .prompt()
                    .unwrap(),
            );
            if self.display_manager.is_empty()
                || Confirm::new(format!("Use {} display manager ? ", self.display_manager).as_str())
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

    fn configure_keymap(&mut self) -> &mut Self {
        let mut keymap =
            File::create("/etc/vconsole.conf").expect("failed to create vconsole.conf");
        assert!(keymap
            .write(
                format!(
                    "KEYMAP={}\nXKBLAYOUT={}\nXKBMODEL={}\nXKBOPTIONS={}",
                    self.keymap, self.keymap_layout, self.keymap_model, self.keymap_options
                )
                    .as_bytes()
            )
            .is_ok());
        assert!(keymap.sync_all().is_ok());
        self
    }

    fn configure_locales(&mut self) -> &mut Self {
        self
    }

    fn configure_hostname(&mut self) -> &mut Self {
        let mut hostname = File::create("/etc/hostname").expect("failed to create hostname");
        assert!(hostname.write(self.hostname.as_bytes()).is_ok());
        assert!(hostname.sync_all().is_ok());
        self
    }

    fn configure_mirrors(&mut self) -> &mut Self {
        assert!(Command::new("reflector")
            .arg("-c")
            .arg(self.mirror_country.as_str())
            .arg("--sort")
            .arg(self.mirror_sort.as_str())
            .arg("--save")
            .arg("/etc/pacman.d/mirrorlist")
            .arg("-p")
            .arg(self.mirror_protocol.as_str())
            .spawn()
            .expect("reflector not founded")
            .wait()
            .expect("")
            .success());
        self
    }

    fn configure_timezone(&mut self) -> &mut Self {
        assert!(Command::new("ln")
            .arg("-s")
            .arg("-f")
            .arg("-v")
            .arg(self.timezone.as_str())
            .arg("/etc/localtime")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }

    fn configure_display_manager(&mut self) -> &mut Self {
        assert!(Command::new("systemctl")
            .arg("enable")
            .arg(self.display_manager.as_str())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        if self.display_manager.eq("lightdm") {
            assert!(Command::new("paru")
                .arg("-S")
                .arg("--noconfirm")
                .args(DISPLAY_MANAGER)
                .spawn()
                .unwrap()
                .wait()
                .unwrap()
                .success());
        }
        self
    }
    fn configure_profiles(&mut self) -> &mut Self {
        for profile in &self.profiles {
            self.install_profile(profile.as_str());
        }
        self
    }

    fn install_profile(&mut self, p: &str) {
        let _ = match p {
            GNOME_DESKTOP => {
                self.install_gnome();
            }
            KDE_DESKTOP => {
                self.install_kde();
            }
            CINNAMON_DESKTOP => {
                self.install_cinnamon();
            }
            CUTEFISH_DESKTOP => {
                self.install_cutefish();
            }
            XFCE_DESKTOP => {
                self.install_xfce();
            }
            QTILE_WINDOW_MANAGER => {
                self.install_qtile();
            }
            I3_WINDOW_MANAGER => {
                self.install_i3();
            }
            PRINTING => {}
            _ => {
                self.choose_profiles();
            }
        };
    }
}

impl Desktop for Arch {
    fn install_gnome(&mut self) -> &mut Self {
        self.install(GNOME.as_ref(), "gdm")
    }

    fn install_cutefish(&mut self) -> &mut Self {
        self.install(CUTEFISH.as_ref(), "lightdm")
    }

    fn install_deepin(&mut self) -> &mut Self {
        self.install(DEEPIN.as_ref(), "lightdm")
    }

    fn install(&mut self, p: &[&str], display_manager: &str) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(p)
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        assert!(Command::new("systemctl")
            .arg("enable")
            .arg(display_manager)
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }
    fn install_kde(&mut self) -> &mut Self {
        self.install(KDE.as_ref(), "lightdm")
    }
    fn install_cinnamon(&mut self) -> &mut Self {
        self.install(CINNAMON.as_ref(), "lightdm")
    }
    fn install_budgie(&mut self) -> &mut Self {
        self.install(BUDGIE.as_ref(), "lightdm")
    }
    fn install_xfce(&mut self) -> &mut Self {
        self.install(XFCE.as_ref(), "lightdm")
    }
    fn install_lxqt(&mut self) -> &mut Self {
        self.install(LXQT.as_ref(), "lightdm")
    }
}

impl WindowManager for Arch {
    fn install_i3(&mut self) -> &mut Self {
        self.install(I3.as_ref(), "lightdm")
    }

    fn install_awesome(&mut self) -> &mut Self {
        self.install(AWESOME.as_ref(), "lightdm")
    }

    fn install_xmonad(&mut self) -> &mut Self {
        self.install(XMONAD.as_ref(), "lightdm")
    }

    fn install_bspwm(&mut self) -> &mut Self {
        self.install(BSPWM.as_ref(), "lightdm")
    }

    fn install_qtile(&mut self) -> &mut Self {
        self.install(QTILE.as_ref(), "lightdm")
    }

    fn install_hyprland(&mut self) -> &mut Self {
        self.install(HYPRLAND.as_ref(), "lightdm")
    }
}

impl Hacking for Arch {
    fn install_hack(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(HACK.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }
    fn install_openssh(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(SSH.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }

    fn install_printing(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(PRINTING.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }
}

impl Diy for Arch {
    fn install_diy(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(DIY.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }
}

impl Languages for Arch {
    fn install_php(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(PHP.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }

    fn install_c(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(C.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }

    fn install_d(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(D.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }

    fn install_r(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(R.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }

    fn install_rust(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(RUST.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }

    fn install_go(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(GO.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }

    fn install_python(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(PYTHON.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }

    fn install_assembly(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(ASSEMBLY.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }
}

impl Server for Arch {
    fn install_cockpit(&mut self) -> &mut Self {
        assert!(Command::new("paru")
            .arg("-S")
            .arg("--noconfirm")
            .args(ADMIN.as_ref())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        self
    }
}
