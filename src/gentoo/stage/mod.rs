use inquire::{Editor, Select};
use reqwest::{get, Error};
use std::fs::{read_to_string, File};
use std::io::{copy, BufReader};
use std::path::Path;
use tar::Archive;
use xz2::read::XzDecoder;

pub struct Stage {
    name: String,
    url: String,
    available: Vec<String>,
}

impl Stage {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            url: "".to_string(),
            available: Vec::new(),
        }
    }
    pub fn select(&mut self) -> &mut Self {
        self.url.clear();
        self.name.clear();
        self.available.clear();
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-openrc-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-desktop-openrc-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-nomultilib-openrc-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-x32-openrc-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-llvm-openrc-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-hardened-openrc-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-hardened-selinux-openrc-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-musl-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-musl-llvm-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-musl-hardened-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-openrc-splitusr-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-systemd-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-desktop-systemd-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-nomultilib-systemd-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-x32-systemd-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/amd64/autobuilds/20240923T191858Z/stage3-amd64-llvm-systemd-20240923T191858Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/x86/autobuilds/20240923T170354Z/stage3-i686-openrc-20240923T170354Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/x86/autobuilds/20240923T170354Z/stage3-i486-openrc-20240923T170354Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/x86/autobuilds/20240923T170354Z/stage3-i686-hardened-openrc-20240923T170354Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/x86/autobuilds/20240923T170354Z/stage3-i686-musl-20240923T170354Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/x86/autobuilds/20240923T170354Z/stage3-i686-systemd-20240923T170354Z.tar.xz"));
        self.available.push(String::from("https://distfiles.gentoo.org/releases/x86/autobuilds/20240923T170354Z/stage3-i486-systemd-20240923T170354Z.tar.xz"));

        self.url = Select::new("", self.available.to_vec())
            .prompt()
            .expect("Could no get selection");
        self.name = String::from("Stage 3");
        self
    }
    pub async fn download(&mut self) -> Result<(), Error> {
        let destination = Path::new("/mnt/gentoo");
        println!("Downloading {} from {}", self.name, self.url);
        let response = get(&self.url).await.expect("");
        let mut file = File::create("/tmp/stage3.tar.xz").expect("Could no create file");
        copy(&mut response.bytes().await.as_ref(), &mut file).expect("Could copy file");
        println!("Download complete!");
        println!("Extracting stage3 to /mnt/gentoo...");
        if !destination.exists() {
            println!("/mnt/gentoo does not exist, creating it...");
            std::fs::create_dir_all(destination).expect("Could create created dir");
        }
        let file = File::open("/tmp/stage3.tar.xz").expect("Could no open file");
        let buf_reader = BufReader::new(file);
        let xz_decoder = XzDecoder::new(buf_reader);
        let mut archive = Archive::new(xz_decoder);
        archive
            .unpack("/mnt/gentoo")
            .expect("Could no extract file");
        println!("Extraction complete!");
        let make = Editor::new("").with_editor_command("vim".as_ref()).with_predefined_text(read_to_string("/mnt/gentoo/etc/portage/make.conf").expect("failed to read make.conf").as_str()).prompt().expect("Could no get editor prompt");
        Ok(())
    }
}
