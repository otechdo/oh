- [PDF](https://raw.githubusercontent.com/otechdo/arch/main/arch.pdf) 

- [EPUB](https://raw.githubusercontent.com/otechdo/arch/main/arch.epub)

> sha512sum arch.pdf

`269036e9752ef3edb89d8ec0b351bc745d25499b45a0c3f9038ed97b8c6b3caa9f9b7af4d08ef71cab74148b67199e73d221f5dffa6ed6aab50ca0010940f703 arch.pdf`

> sha512sum arch.epub

`8ddf24144aeb1abfe16d3500ccefa72ab46c9a513dbdc76c3b34d387bc76d66640d54f77a5ce26facde6012110ef9fb257bed04ec87edf087416acfceb9bcb46 arch.epub`

# Set keymap

```bash
loadkeys <keymap>
```

## Partitioning

1. 1024MB  EFI partition         # ef00      /boot/efi    
2. 4096MB  Linux partition    # 8300     /boot
3. 100%       Linux partition    # 8300     /

```bash
cgdisk /dev/sda
```

## Formatting

> /boot/efi

```bash
mkfs.vfat -F 32 /dev/sda1
```

> /boot

```bash
mkfs.ext2 /dev/sda2
```

> /

```bash
mkfs.ext4 /dev/sda3
```

## Mounting

### The root partition

```bash
mount /dev/sda3 /mnt
```

### Create the boot mount point

```bash
mkdir /mnt/boot
```

### Mounting the boot partition

```bash
mount /dev/sda2 /mnt/boot
```

### Create the EFI mount point

```bash
mkdir /mnt/boot/efi
```

### Mounting the EFI partition

```bash
mount /dev/sda1 /mnt/boot/efi
```

## Update mirrorlist

```bash
reflector -c <country> --sort delay --save /etc/pacman.d/mirrorlist -p https
```

## Init pacman

```bash
pacman-key --init && pacman-key --populate archlinux
```

# Installation

## The base system

```bash
pacstrap /mnt base base-devel wget git linux linux-firmware vim efibootmgr rustup sudo grub networkmanager w3m archiso reflector <shell> <ucode> <graphics_driver>
```

# Generate fstab

```bash
genfstab -U /mnt >> /mnt/etc/fstab
```

# Enter in the new system

```bash
arch-chroot /mnt && cd ~
```

# Manage accounts

## Create your account

```bash
useradd -m -U -c 'YOUR REAL NAME' -s <shell> <username>
```

## Generate root password

```bash
passwd root
```

### Generate your password

```bash
passwd <username>
```

### Add your account to sudoers file

```bash
echo '<username> ALL=(ALL) ALL' > /etc/sudoers.d/<username>
```

# Sign in

```bash
su - <username>
```

# Configure rust

```bash
rustup default stable
```

# Modify pacman.conf

```bash
sudo vim /etc/pacman.conf
```

# Refresh repositories

```bash
sudo pacman -Sy
```

# Installation of yay

```bash
git clone https://aur.archlinux.org/yay 
cd yay
makepkg -si 
cd ..
rm -rf yay
```

# Install arch

## From GitHub

```bash
git clone https://github.com/otechdo/arch 
cd arch 
make 
sudo make install
```

### From Crates.io

```bash
cargo install arch 
install -m 755 "$HOME/.cargo/bin/arch" /usr/bin/arch
```

### From Aur

```bash
paru -Syu manager
```

## Setup a new arch

```bash
arch --setup
```

## Desktop

- [@deepin](https://wiki.archlinux.org/title/Deepin_Desktop_Environment)
- [@kde](https://wiki.archlinux.org/title/KDE)
- [@gnome](https://wiki.archlinux.org/title/GNOME)
- [@xmonad](https://wiki.archlinux.org/title/xmonad)
- [@i3](https://wiki.archlinux.org/title/i3)

## Install all selected packages on arch

```bash
arch --install
arch -S <pkg> <pkg>
```

# Quit the fresh new system

```bash
exit
```

# Umount all mounted partitions

```bash
umount -R /mnt
```

# Reboot

```bash
reboot
```

# Arch commands

## Setup a new arch

```bash
arch -i
```

```bash
arch --setup
```

## Remove packages

```bash
arch -R <pkg> <pkg>
```

```bash
arch --uninstall
```

## Install new packages

```bash
arch -S <pkg> <pkg>
```

```bash
arch --install
```

## Update mirrorlist

```bash
arch -M
```

```bash
arch --mirrors
```

## Check updates

```bash
arch -C
```

```bash
arch --check
```

## Install packages as dependencies

```bash
arch -d
```

```bash
arch --deps
```

# Update archlinux

```bash
arch
```

```bash
arch -u
```

```bash
arch --update
```

## Search a package

```bash
arch -s <pkg>
```

```bash
arch --search <pkg>
```

## Show arch current version

```bash
arch -v
```

```bash
arch --version
```

## Download updates

```bash
arch -d
```

```bash
arch --download-updates
```

## Show help message

```bash
arch -h
```

```bash
arch --help
```

## Cancel the upgrade reboot

```bash
arch -x
```

```bash
arch --cancel
```

## Upgrade the system and reboot

```bash
arch -U
```

```bash
arch --upgrade
```

## Generate arch packages cache

```bash
arch -c
```

```bash
arch --cache
```

## Navigate on news

```bash
arch -n
```

```bash
arch --news
```

## Navigate on the Aur

```bash
arch -a
```

```bash
arch --aur
```

### Navigate on the forum

```bash
arch -f
```

```bash
arch --forum
```

### Navigate on the man pages

```bash
arch -m
```

```bash
arch --man
```

```bash
arch --woman
```

### Navigate on the wiki

```bash
arch -w
```

```bash
arch --wiki
```

## Host Distros

Distrobox has been successfully tested on:

| Distro                       | Version                                                                                 | Notes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ---------------------------- | --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Alpine Linux                 |                                                                                         | To setup rootless podman, look [HERE](https://wiki.alpinelinux.org/wiki/Podman)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Arch Linux                   |                                                                                         | `distrobox` and `distrobox-git` are available in AUR (thanks [M0Rf30](https://github.com/M0Rf30)!).<br>To setup rootless podman, look [HERE](https://wiki.archlinux.org/title/Podman)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Bazzite                      | 38                                                                                      | `distrobox-git` is preinstalled.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| CentOS                       | 8<br>8 Stream<br>9 Stream                                                               | `distrobox` is available in epel repos. (thanks [alcir](https://github.com/alcir)!)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ChromeOS                     | Debian 11 (docker with make-shared workaround #non-shared-mounts)<br>Debian 12 (podman) | using built-in Linux on ChromeOS mode which is debian-based, which can be [upgraded](https://wiki.debian.org/DebianUpgrade) from 11 bullseye to 12 bookworm (in fact 12 is recommended)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Debian                       | 11<br>12<br>Testing<br>Unstable                                                         | `distrobox` is available in default repos starting from version 12 (thanks [michel-slm!](https://github.com/michel-slm!)!)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| deepin                       | 23<br>Testing<br>Unstable                                                               | `distrobox` is available in default repos in `testing` and `unstable`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| EndlessOS                    | 4.0.0                                                                                   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Fedora Silverblue/Kinoite    | 35<br>36<br>37<br>Rawhide                                                               | `distrobox` is available in default repos.(thanks [alcir](https://github.com/alcir)!)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Fedora                       | 35<br>36<br>37<br>38<br>Rawhide                                                         | `distrobox` is available in default repos.(thanks [alcir](https://github.com/alcir)!)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Gentoo                       |                                                                                         | To setup rootless podman, look [HERE](https://wiki.gentoo.org/wiki/Podman)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| KDE neon                     |                                                                                         | `distrobox` is available in default repo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| Manjaro                      |                                                                                         | To setup rootless podman, look [HERE](https://wiki.archlinux.org/title/Podman)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| NixOS                        | 21.11                                                                                   | Make sure to mind your executable paths. Sometimes a container will not have nix paths, and sometimes it will not have its own paths.<br>Distrobox is available in Nixpkg collection (thanks [AtilaSaraiva](https://github.com/AtilaSaraiva)!)<<br>To setup Docker, look [HERE](https://nixos.wiki/wiki/Docker)<br>To setup Podman, look [HERE](https://nixos.wiki/wiki/Podman) and [HERE](https://gist.github.com/adisbladis/187204cb772800489ee3dac4acdd9947)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| openSUSE                     | Leap 15.4<br>Leap 15.3<br>Leap 15.2                                                     | Packages are available [here](https://software.opensuse.org/download/package?package=distrobox&project=home%3Adfaggioli%3Amicroos-desktop) (thanks [dfaggioli](https://github.com/dfaggioli)!).<br>To install on openSUSE Leap 15, Use the following repository links in the `zypper addrepo` command: [15.4](https://download.opensuse.org/repositories/home:dfaggioli:microos-desktop/15.4/home:dfaggioli:microos-desktop.repo), [15.3](https://download.opensuse.org/repositories/home:dfaggioli:microos-desktop/15.3/home:dfaggioli:microos-desktop.repo), [15.2](https://download.opensuse.org/repositories/home:dfaggioli:microos-desktop/15.2/home:dfaggioli:microos-desktop.repo). Then:<br>`zypper refresh && zypper install distrobox`.<br>`Podman` under SUSE Leap, cannot initialize correctly the containers managed by `distrobox` until [this openSUSE bug](https://bugzilla.opensuse.org/show_bug.cgi?id=1199871) is fixed, or `podman` logging is configured properly. |
| openSUSE                     | Tumbleweed MicroOS                                                                      | `distrobox` is available in default repos (thanks [dfaggioli](https://github.com/dfaggioli)!)<br>For Tumbleweed, do: `zypper install distrobox`.<br>For MicroOS, **distrobox is installed by default**.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| SUSE Linux Enterprise Server | 15 Service Pack 4<br>15 Service Pack 3<br>15 Service Pack 2                             | Same procedure as the one for openSUSE (Leap, respective versions, of course). Use the following repository links in the `zypper addrepo` command: [SLE-15-SP4](https://download.opensuse.org/repositories/home:dfaggioli:microos-desktop/15.4/home:dfaggioli:microos-desktop.repo), [SLE-15-SP3](https://download.opensuse.org/repositories/home:dfaggioli:microos-desktop/15.3/home:dfaggioli:microos-desktop.repo), [SLE-15-SP4](https://download.opensuse.org/repositories/home:dfaggioli:microos-desktop/SLE_15_SP2/home:dfaggioli:microos-desktop.repo). Then:<br>`zypper refresh && zypper install distrobox`.<br>`Podman` under SUSE Leap, cannot initialize correctly the containers managed by `distrobox` until [this openSUSE bug](https://bugzilla.opensuse.org/show_bug.cgi?id=1199871) is fixed, or `podman` logging is configured properly.                                                                                                                             |
| SteamOS                      |                                                                                         | You can follow the [Install Podman in a static manner](https://distrobox.it/posts/install_podman_static/) or [Install Lilipod in a static manner](https://distrobox.it/posts/install_lilipod_static/) guide, this will install it in your $HOME and it will survive updates.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| RedHat                       | 8<br>9                                                                                  | `distrobox` is available in epel repos. (thanks [alcir](https://github.com/alcir)!)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Ubuntu                       | 18.04<br>20.04<br>22.04<br>22.10<br>23.04<br>                                           | Older versions based on 20.04 or earlier may need external repos to install newer Podman and Docker releases.<br>Derivatives like Pop_OS!, Mint and Elementary OS should work the same.<br>[Now PPA available!](https://launchpad.net/~michel-slm/+archive/ubuntu/distrobox), also `distrobox` is available in default repos from `22.10` onward (thanks [michel-slm](https://github.com/michel-slm)!)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Vanilla OS                   | 22.10<br>Orchid                                                                         | `distrobox` should be installed in the home directory using the official script                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Void Linux                   | glibc                                                                                   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Windows                      | Oracle Linux 9                                                                          | using built-in Windows Subsystem for Linux                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

## Containers Distros

Distrobox guests tested successfully with the following container images:

| Distro                | Version                                            | Images                                                                                                                                                                                                                                                                                                                  |
| --------------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AlmaLinux (Toolbox)   | 8<br>9                                             | quay.io/toolbx-images/almalinux-toolbox:8<br>quay.io/toolbx-images/almalinux-toolbox:9<br>quay.io/toolbx-images/almalinux-toolbox:latest                                                                                                                                                                                |
| Alpine (Toolbox)      | 3.16<br>3.17<br>3.18<br>edge                       | quay.io/toolbx-images/alpine-toolbox:3.16<br>quay.io/toolbx-images/alpine-toolbox:3.17<br>quay.io/toolbx-images/alpine-toolbox:3.18<br>quay.io/toolbx-images/alpine-toolbox:edge<br>quay.io/toolbx-images/alpine-toolbox:latest                                                                                         |
| AmazonLinux (Toolbox) | 2<br>2022                                          | quay.io/toolbx-images/amazonlinux-toolbox:2<br>quay.io/toolbx-images/amazonlinux-toolbox:2023<br>quay.io/toolbx-images/amazonlinux-toolbox:latest                                                                                                                                                                       |
| Archlinux (Toolbox)   |                                                    | quay.io/toolbx/arch-toolbox:latest                                                                                                                                                                                                                                                                                      |
| Bazzite Arch          |                                                    | ghcr.io/ublue-os/bazzite-arch:latest<br>ghcr.io/ublue-os/bazzite-arch-gnome:latest                                                                                                                                                                                                                                      |
| Centos (Toolbox)      | stream8<br>stream9                                 | quay.io/toolbx-images/centos-toolbox:stream8<br>quay.io/toolbx-images/centos-toolbox:stream9<br>quay.io/toolbx-images/centos-toolbox:latest                                                                                                                                                                             |
| Debian (Toolbox)      | 10<br>11<br>12<br>testing<br>unstable<br>          | quay.io/toolbx-images/debian-toolbox:10<br>quay.io/toolbx-images/debian-toolbox:11<br>quay.io/toolbx-images/debian-toolbox:12<br>quay.io/toolbx-images/debian-toolbox:testing<br>quay.io/toolbx-images/debian-toolbox:unstable<br>quay.io/toolbx-images/debian-toolbox:latest                                           |
| Fedora (Toolbox)      | 37<br>38<br>39<br>Rawhide                          | registry.fedoraproject.org/fedora-toolbox:37<br>registry.fedoraproject.org/fedora-toolbox:38<br>registry.fedoraproject.org/fedora-toolbox:39<br>registry.fedoraproject.org/fedora-toolbox:rawhide                                                                                                                       |
| openSUSE (Toolbox)    |                                                    | registry.opensuse.org/opensuse/distrobox:latest                                                                                                                                                                                                                                                                         |
| RedHat (Toolbox)      | 8<br>9                                             | registry.access.redhat.com/ubi8/toolbox<br>registry.access.redhat.com/ubi9/toolbox<br>quay.io/toolbx-images/rhel-toolbox:latest                                                                                                                                                                                         |
| Rocky Linux (Toolbox) | 8<br>9                                             | quay.io/toolbx-images/rockylinux-toolbox:8<br>quay.io/toolbx-images/rockylinux-toolbox:9<br>quay.io/toolbx-images/rockylinux-toolbox:latest                                                                                                                                                                             |
| Ubuntu (Toolbox)      | 16.04<br>18.04<br>20.04<br>22.04                   | quay.io/toolbx/ubuntu-toolbox:16.04<br>quay.io/toolbx/ubuntu-toolbox:18.04<br>quay.io/toolbx/ubuntu-toolbox:20.04<br>quay.io/toolbx/ubuntu-toolbox:22.04<br>quay.io/toolbx/ubuntu-toolbox:latest                                                                                                                        |
|                       |                                                    |                                                                                                                                                                                                                                                                                                                         |
| AlmaLinux             | 8<br>8-minimal<br>9<br>9-minimal                   | docker.io/library/almalinux:8<br>docker.io/library/almalinux:9                                                                                                                                                                                                                                                          |
| Alpine Linux          | 3.15<br>3.16                                       | docker.io/library/alpine:3.15<br>docker.io/library/alpine:3.16<br>docker.io/library/alpine:latest                                                                                                                                                                                                                       |
| AmazonLinux           | 1<br>2<br>2023                                     | public.ecr.aws/amazonlinux/amazonlinux:1<br>public.ecr.aws/amazonlinux/amazonlinux:2<br>public.ecr.aws/amazonlinux/amazonlinux:2023                                                                                                                                                                                     |
| Archlinux             |                                                    | docker.io/library/archlinux:latest                                                                                                                                                                                                                                                                                      |
| CentOS Stream         | 8<br>9                                             | quay.io/centos/centos:stream8<br>quay.io/centos/centos:stream9                                                                                                                                                                                                                                                          |
| CentOS                | 7                                                  | quay.io/centos/centos:7                                                                                                                                                                                                                                                                                                 |
| Chainguard Wolfi      | Small note: sudo is missing, use su-exec instead.  | cgr.dev/chainguard/wolfi-base:latest                                                                                                                                                                                                                                                                                    |
| ClearLinux            |                                                    | docker.io/library/clearlinux:latest<br>docker.io/library/clearlinux:base                                                                                                                                                                                                                                                |
| Crystal Linux         |                                                    | registry.getcryst.al/crystal/misc/docker:latest                                                                                                                                                                                                                                                                         |
| Debian                | 7<br>8<br>9<br>10<br>11<br>12                      | docker.io/debian/eol:wheezy<br>docker.io/library/debian:buster-backports<br>docker.io/library/debian:bullseye-backports<br>docker.io/library/debian:bookworm-backports<br>docker.io/library/debian:stable-backports                                                                                                     |
| Debian                | Testing                                            | docker.io/library/debian:testing<br>docker.io/library/debian:testing-backports                                                                                                                                                                                                                                          |
| Debian                | Unstable                                           | docker.io/library/debian:unstable                                                                                                                                                                                                                                                                                       |
| deepin                | 20 (apricot)<br>23 (beige)                         | docker.io/linuxdeepin/apricot                                                                                                                                                                                                                                                                                           |
| Fedora                | 36<br>37<br>38<br>39<br>Rawhide                    | quay.io/fedora/fedora:36<br>quay.io/fedora/fedora:37<br>quay.io/fedora/fedora:38<br>quay.io/fedora/fedora:39<br>quay.io/fedora/fedora:rawhide                                                                                                                                                                           |
| Gentoo Linux          | rolling                                            | docker.io/gentoo/stage3:latest                                                                                                                                                                                                                                                                                          |
| KDE neon              | Latest                                             | invent-registry.kde.org/neon/docker-images/plasma:latest                                                                                                                                                                                                                                                                |
| Kali Linux            | rolling                                            | docker.io/kalilinux/kali-rolling:latest                                                                                                                                                                                                                                                                                 |
| Mint                  | 21.1                                               | docker.io/linuxmintd/mint21.1-amd64                                                                                                                                                                                                                                                                                     |
| Neurodebian           | nd100                                              | docker.io/library/neurodebian:nd100                                                                                                                                                                                                                                                                                     |
| openSUSE              | Leap                                               | registry.opensuse.org/opensuse/leap:latest                                                                                                                                                                                                                                                                              |
| openSUSE              | Tumbleweed                                         | registry.opensuse.org/opensuse/distrobox:latest<br>registry.opensuse.org/opensuse/tumbleweed:latest<br>registry.opensuse.org/opensuse/toolbox:latest                                                                                                                                                                    |
| Oracle Linux          | 7<br>7-slim<br>8<br>8-slim<br>9<br>9-slim          | container-registry.oracle.com/os/oraclelinux:7<br>container-registry.oracle.com/os/oraclelinux:7-slim<br>container-registry.oracle.com/os/oraclelinux:8<br>container-registry.oracle.com/os/oraclelinux:8-slim<br>container-registry.oracle.com/os/oraclelinux:9<br>container-registry.oracle.com/os/oraclelinux:9-slim |
| RedHat (UBI)          | 7<br>8<br>9                                        | registry.access.redhat.com/ubi7/ubi<br>registry.access.redhat.com/ubi8/ubi<br>registry.access.redhat.com/ubi8/ubi-init<br>registry.access.redhat.com/ubi8/ubi-minimal<br>registry.access.redhat.com/ubi9/ubi<br>registry.access.redhat.com/ubi9/ubi-init<br>registry.access.redhat.com/ubi9/ubi-minimal                 |
| Rocky Linux           | 8<br>8-minimal<br>9                                | quay.io/rockylinux/rockylinux:8<br>quay.io/rockylinux/rockylinux:8-minimal<br>quay.io/rockylinux/rockylinux:9<br>quay.io/rockylinux/rockylinux:latest                                                                                                                                                                   |
| Scientific Linux      | 7                                                  | docker.io/library/sl:7                                                                                                                                                                                                                                                                                                  |
| SteamOS               |                                                    | ghcr.io/linuxserver/steamos:latest                                                                                                                                                                                                                                                                                      |
| Ubuntu                | 14.04<br>16.04<br>18.04<br>20.04<br>22.04<br>23.04 | docker.io/library/ubuntu:14.04<br>docker.io/library/ubuntu:16.04<br>docker.io/library/ubuntu:18.04<br>docker.io/library/ubuntu:20.04<br>docker.io/library/ubuntu:22.04                                                                                                                                                  |
| Vanilla OS            | VSO                                                | ghcr.io/vanilla-os/vso:main                                                                                                                                                                                                                                                                                             |
| Void Linux            |                                                    | ghcr.io/void-linux/void-glibc-full:latest                                                                                                                                                                                                                                                                               |

# Toolbox support

[GitHub - toolbx-images/images: Community maintained container images to use with toolbx and distrobox](https://github.com/toolbx-images/images?tab=readme-ov-file)

## List all toolbox

```bash
os --list
```

## Create a new toolbox

```bash
os --add fedora 39 workstation
os --add fedora 39
```

## Create a new toolbox from an image

```bash
os --add-from quay.io/toolbx-images/debian-toolbox:12
```

## Enter in toolbox

```bash
os --use workstation
os --use fedora-toolbox-39
```

## Run a command in toolbox

```bash
os --run workstation ls
os --run fedora-toolbox-39 ls
```

## Stop a toolbox

```bash
os --stop workstation
os --stop fedora-toolbox-39
```

## Remove a toolbox

```bash
os --rm workstation
os --rm fedora-toolbox-39
```

## Key Bindings

This file lists all of the key bindings currently registered by prompts.

## All prompts

These key bindings may be used with all prompts.

| **command**                      | **description**         |
| -------------------------------- | ----------------------- |
| <kbd>enter</kbd>                 | Submit answer.          |
| <kbd>esc</kbd>                   | Cancel the prompt\*.    |
| <kbd>ctrl</kbd>  +  <kbd>c</kbd> | Interrupt the prompt\*. |

\* Canceling and interrupting a prompt have two different meanings. Canceling is defined specially for when the end user is allowed to skip a prompt, the library user can then use `prompt_skippable` which wraps the return type into an `Option` and catches the `CanceledOperation` error transforming it into a `Ok(None)` result. Interrupted operations are closer to "stop-the-world" operations, where the library user should treat them as termination commands.

## Text Input

These key bindings may be used with all prompts that ask the user for text input: [`Text`], [`Select`], [`MultiSelect`], [`Confirm`], [`CustomType`] and [`Password`]. The [`Editor`] prompt is not included because it opens a separate text editor for text input.

| **command**                         | **description**                                 |
| ----------------------------------- | ----------------------------------------------- |
| <kbd>character</kbd>                | Insert the character into the input.            |
| <kbd>left</kbd>                     | Move the cursor back one character.             |
| <kbd>right</kbd>                    | Move the cursor forward one character.          |
| <kbd>ctrl</kbd> + <kbd>left</kbd>   | Move one word to the left of the cursor.        |
| <kbd>ctrl</kbd> + <kbd>right</kbd>  | Move one word to the right of the cursor.       |
| <kbd>home</kbd>                     | Move cursor to the start of the line*.          |
| <kbd>end</kbd>                      | Move cursor to the end of the line*.            |
| <kbd>backspace</kbd>                | Delete one character to the left of the cursor. |
| <kbd>delete</kbd>                   | Delete the character at the cursor.             |
| <kbd>ctrl</kbd> + <kbd>delete</kbd> | Delete one word to the right of the cursor.     |

\* Key bindings not supported on [`Select`] and [`MultiSelect`] prompts.

## Text Prompts

These key bindings may be used in [`Text`] prompts.

| **command**          | **description**                                               |
| -------------------- | ------------------------------------------------------------- |
| <kbd>enter</kbd>     | Submit the current current text input.                        |
| <kbd>up</kbd>        | When suggestions are displayed, move cursor one row up.       |
| <kbd>down</kbd>      | When suggestions are displayed, move cursor one row down.     |
| <kbd>page up</kbd>   | When suggestions are displayed, move cursor one page up.      |
| <kbd>page down</kbd> | When suggestions are displayed, move cursor one page down.    |
| <kbd>tab</kbd>       | Replace current input with the resulting suggestion if any.   |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## Select Prompts

These key bindings may be used in [`Select`] prompts.

| **command**          | **description**                                               |
| -------------------- | ------------------------------------------------------------- |
| <kbd>enter</kbd>     | Submit the current highlighted option.                        |
| <kbd>up</kbd>        | Move cursor one row up.                                       |
| <kbd>down</kbd>      | Move cursor one row down.                                     |
| <kbd>k</kbd>         | Move cursor one row up when vim mode is enabled.              |
| <kbd>j</kbd>         | Move cursor one row down when vim mode is enabled.            |
| <kbd>page up</kbd>   | Move cursor one page up.                                      |
| <kbd>page down</kbd> | Move cursor one page down.                                    |
| <kbd>home</kbd>      | Move cursor to the first option.                              |
| <kbd>end</kbd>       | Move cursor to the last option.                               |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## MultiSelect Prompts

These key bindings may be used in [`MultiSelect`] prompts.

| **command**          | **description**                                               |
| -------------------- | ------------------------------------------------------------- |
| <kbd>enter</kbd>     | Submit the options currently selected.                        |
| <kbd>space</kbd>     | Toggle the selection of the current highlighted option.       |
| <kbd>up</kbd>        | Move cursor one row up.                                       |
| <kbd>down</kbd>      | Move cursor one row down.                                     |
| <kbd>k</kbd>         | Move cursor one row up when vim mode is enabled.              |
| <kbd>j</kbd>         | Move cursor one row down when vim mode is enabled.            |
| <kbd>page up</kbd>   | Move cursor one page up.                                      |
| <kbd>page down</kbd> | Move cursor one page down.                                    |
| <kbd>home</kbd>      | Move cursor to the first option.                              |
| <kbd>end</kbd>       | Move cursor to the last option.                               |
| <kbd>left</kbd>      | Unselect all options.                                         |
| <kbd>right</kbd>     | Select all options.                                           |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## DateSelect Prompts

These key bindings may be used in the interactive calendar of the [`DateSelect`] prompt.

| **command**                              | **description**                                               |
| ---------------------------------------- | ------------------------------------------------------------- |
| <kbd>space bar</kbd> or <kbd>enter</kbd> | Submit the current highlighted date.                          |
| <kbd>up</kbd>                            | Move cursor one row up.                                       |
| <kbd>down</kbd>                          | Move cursor one row down.                                     |
| <kbd>left</kbd>                          | Move cursor one column to the left.                           |
| <kbd>right</kbd>                         | Move cursor one column to the right.                          |
| <kbd>k</kbd>                             | Move cursor one row up when vim mode is enabled.              |
| <kbd>j</kbd>                             | Move cursor one row down when vim mode is enabled.            |
| <kbd>h</kbd>                             | Move cursor one column to the left when vim mode is enabled.  |
| <kbd>l</kbd>                             | Move cursor one column to the right when vim mode is enabled. |
| <kbd>ctrl</kbd> + <kbd>up</kbd>          | Move calendar back by one year.                               |
| <kbd>ctrl</kbd> + <kbd>down</kbd>        | Move calendar forward by one year.                            |
| <kbd>ctrl</kbd> + <kbd>left</kbd>        | Move calendar back by one month.                              |
| <kbd>ctrl</kbd> + <kbd>right</kbd>       | Move calendar forward by one month.                           |

## Editor Prompts

These key bindings may be used in [`Editor`] prompts.

| **command**      | **description**                                                |
| ---------------- | -------------------------------------------------------------- |
| <kbd>e</kbd>     | Open the editor.                                               |
| <kbd>enter</kbd> | Submit the current content of the temporary file being edited. |
