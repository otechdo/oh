use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process::Command;
use std::time::Instant;
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
const BOOT: &str = "/usr/share/oh/conf/boot";
const SHELLS: &str = "/usr/share/oh/conf/shells";

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

fn main() {
    let app = Application::builder()
        .application_id("com.otechdo.oh")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Oh")
            .default_width(400)
            .default_height(200)
            .build();
        window.present();
    });
    app.run();
}