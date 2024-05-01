<!-- TOC -->
* [Arch](#arch)
* [Command line interface](#command-line-interface)
  * [What it's?](#what-its)
  * [Why ?](#why-)
  * [Donate](#donate)
  * [Documentation](#documentation)
* [Key bindings](#key-bindings)
  * [All prompts](#all-prompts)
  * [Text input](#text-input)
  * [Text prompts](#text-prompts)
  * [Select prompts](#select-prompts)
  * [Multi select prompts](#multi-select-prompts)
  * [Date select prompts](#date-select-prompts)
  * [Editor prompts](#editor-prompts)
* [Installation](#installation)
  * [Set keymap](#set-keymap)
  * [Partitioning](#partitioning)
  * [Update the pacman mirrors](#update-the-pacman-mirrors)
  * [Initialise the pacman keyring](#initialise-the-pacman-keyring)
  * [Install the base system](#install-the-base-system)
    * [Uefi system](#uefi-system)
    * [Bios system](#bios-system)
  * [Install the arch dependencies](#install-the-arch-dependencies)
    * [Setup mode](#setup-mode)
    * [After install mode](#after-install-mode)
  * [Generate fstab](#generate-fstab)
  * [Enter in the new system](#enter-in-the-new-system)
  * [Manage accounts](#manage-accounts)
    * [Create your account](#create-your-account)
    * [Generate root password](#generate-root-password)
    * [Generate your password](#generate-your-password)
    * [Add your account to sudoers file](#add-your-account-to-sudoers-file)
  * [Sign in](#sign-in)
    * [Configure rust](#configure-rust)
    * [Modify the pacman configuration](#modify-the-pacman-configuration)
    * [Refresh repositories](#refresh-repositories)
    * [Paru](#paru)
    * [Arch](#arch-1)
      * [From GitHub](#from-github)
      * [From Aur](#from-aur)
* [Supported desktop](#supported-desktop)
* [Supported window manager](#supported-window-manager)
    * [Run the setup](#run-the-setup)
      * [Rebuild a complete arch](#rebuild-a-complete-arch)
      * [Quit the fresh new system](#quit-the-fresh-new-system)
      * [Unmounted all partitions](#unmounted-all-partitions)
      * [Reboot](#reboot)
* [Distrobox management](#distrobox-management)
  * [Distrobox containers](#distrobox-containers-)
    * [List all boxes](#list-all-boxes)
    * [Create a new box](#create-a-new-box)
    * [Enter inside a box](#enter-inside-a-box)
    * [Stop a box](#stop-a-box)
    * [Stop all boxes](#stop-all-boxes)
    * [Run a command in a box](#run-a-command-in-a-box)
    * [Stop a box](#stop-a-box-1)
    * [Remove all boxes](#remove-all-boxes)
    * [Remove a box](#remove-a-box)
    * [Upgrade all boxes](#upgrade-all-boxes)
    * [Display help](#display-help)
<!-- TOC -->

# Arch

| Code checkup                                                                                                                             | Continuous testing                                                                                                                                                        | version |
|------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------|
| [![zuu](https://github.com/otechdo/arch/actions/workflows/zuu.yml/badge.svg)](https://github.com/otechdo/arch/actions/workflows/zuu.yml) | [![continuous](https://github.com/otechdo/arch/actions/workflows/continuous.yml/badge.svg?branch=main)](https://github.com/otechdo/arch/actions/workflows/continuous.yml) | `1.0.0` |

# Command line interface

| command                            | alias                   | do                                        |
|------------------------------------|-------------------------|-------------------------------------------|
| `arch`                             | `arch`                  | Run installer in update mode              |
| `arch --setup`                     | `arch -s`               | Run installer in first install mode       |
| `arch --new`                       | `arch -n`               | Run installer in new config mode          |
| `arch --install`                   | `arch -i`               | Run arch in packages installer mode       |
| `arch --deps`                      | `arch -d`               | Run arch in dependencies installer mode   |
| `arch --uninstall`                 | `arch -u`               | Run arch in packages uninstaller mode     |
| `arch --mirrors`                   | `arch -m`               | Run arch in mirrors updater mode          |
| `arch --check`                     | `arch -c`               | Run arch in check updates mode            |
| `arch --download`                  | `arch -d`               | Run arch in download iso mode             |
| `arch --help`                      | `arch -h`               | Display the help message                  |
| `arch --cache`                     | `arch -C`               | Generate cache files                      |
| `arch --wiki`                      | `arch -w`               | Navigate to the wiki                      |
| `arch --news`                      | `arch -n`               | Navigate to the news                      |
| `arch --aur`                       | `arch -a`               | Navigate to the aur                       |
| `arch --forum`                     | `arch -f`               | Navigate to the forum                     |
| `arch --man`                       | `arch -m`               | Navigate to the man pages                 |
| `arch --vote <pkg>`                | `arch -v <pkg>`         | Vote for a aur package                    |
| `arch --search pkg`                | `arch -S pkg`           | Run arch in search package mode           |
| `arch --create-usb <device_path> ` | `arch -U <devive_path>` | Create usb with the current configuration |

## What it's?

It's a project to install archlinux manually.

It's requiring to know an archlinux system.

## Why ?

I've started development to customize my arch more simply.

## Donate

![donate](https://raw.githubusercontent.com/otechdo/arch/main/donate.png) 

[@donate](https://www.paypal.com/donate/?hosted_button_id=4EB2UW73HN8Q2)

## Documentation

- [@fr]()
- [@es]()
- [@it]()
- [@official documentation](https://wiki.archlinux.org/title/Installation_guide)

# Key bindings

This file lists all the key bindings currently registered by prompts.

## All prompts

These key bindings may be used with all prompts.

| **command**                      | **description**         |
|----------------------------------|-------------------------|
| <kbd>enter</kbd>                 | Submit answer.          |
| <kbd>esc</kbd>                   | Cancel the prompt\*.    |
| <kbd>ctrl</kbd>  +  <kbd>c</kbd> | Interrupt the prompt\*. |

\* Cancelling and interrupting a prompt have two different meanings.

Cancelling is defined specially for when the end user is allowed to skip a prompt.

The library user can then use `prompt_skippable` which wraps the return type into an `Option` and catches the `CanceledOperation` error transforming it into a `Ok(None)` result.

Interrupted operations are closer to "stop-the-world" operations, where the library user should treat them as termination commands.

## Text input

These key bindings may be used with all prompts that ask the user for text input: [`Text`], [`Select`], [`MultiSelect`], [`Confirm`], [`CustomType`] and [`Password`]. 

The [`Editor`] prompt is not included because it opens a separate text editor for text input.

| **command**                         | **description**                                 |
|-------------------------------------|-------------------------------------------------|
| <kbd>character</kbd>                | Insert the character into the input.            |
| <kbd>left</kbd>                     | Move the cursor back one character.             |
| <kbd>right</kbd>                    | Move the cursor forward one character.          |
| <kbd>ctrl</kbd> + <kbd>left</kbd>   | Move one word to the left of the cursor.        |
| <kbd>ctrl</kbd> + <kbd>right</kbd>  | Move one word to the right of the cursor.       |
| <kbd>home</kbd>                     | Move cursor to the start of the line.           |
| <kbd>end</kbd>                      | Move cursor to the end of the line*.            |
| <kbd>backspace</kbd>                | Delete one character to the left of the cursor. |
| <kbd>delete</kbd>                   | Delete the character at the cursor.             |
| <kbd>ctrl</kbd> + <kbd>delete</kbd> | Delete one word to the right of the cursor.     |

\* Key bindings aren't supported on [`Select`] and [`MultiSelect`] prompts.

## Text prompts

These key bindings may be used in [`Text`] prompts.

| **command**          | **description**                                               |
|----------------------|---------------------------------------------------------------|
| <kbd>enter</kbd>     | Submit the current current text input.                        |
| <kbd>up</kbd>        | When suggestions are displayed, move cursor one row up.       |
| <kbd>down</kbd>      | When suggestions are displayed, move cursor one row down.     |
| <kbd>page up</kbd>   | When suggestions are displayed, move cursor one page up.      |
| <kbd>page down</kbd> | When suggestions are displayed, move cursor one page down.    |
| <kbd>tab</kbd>       | Replace current input with the resulting suggestion if any.   |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## Select prompts

These key bindings may be used in [`Select`] prompts.

| **command**          | **description**                                               |
|----------------------|---------------------------------------------------------------|
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

## Multi select prompts

These key bindings may be used in [`MultiSelect`] prompts.

| **command**          | **description**                                               |
|----------------------|---------------------------------------------------------------|
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
| <kbd>left</kbd>      | Deselect all options.                                         |
| <kbd>right</kbd>     | Select all options.                                           |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## Date select prompts

These key bindings may be used in the interactive calendar of the [`DateSelect`] prompt.

| **command**                              | **description**                                               |
|------------------------------------------|---------------------------------------------------------------|
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

## Editor prompts

These key bindings may be used in [`Editor`] prompts.

| **command**      | **description**                                                |
|------------------|----------------------------------------------------------------|
| <kbd>e</kbd>     | Open the editor.                                               |
| <kbd>enter</kbd> | Submit the current content of the temporary file being edited. |

# Installation

Let's begin the arch installation! 

## Set keymap

```bash
loadkeys <keymap>
```

## Partitioning

```bash
cgdisk /dev/sda
```

| Number | Size  | Code | Name        | Formating sample           | Mount path      | Mount command                   | Description            |
|--------|-------|------|-------------|----------------------------|-----------------|---------------------------------|------------------------|
| 1      | +1G   | EF00 | `/boot/efi` | `mkfs.fat -F 32 /dev/sda1` | `/mnt/boot/efi` | `mount /dev/sda1 /mnt/boot/efi` | The efi boot partition |
| 2      | +4G   | 8300 | `/boot`     | `mkfs.xfs /dev/sda2`       | `/mnt/boot`     | `mount /dev/sda2 /mnt/boot`     | The boot partition     |
| 3      | +100G | 8300 | `/`         | `mkfs.xfs /dev/sda3`       | `/mnt`          | `mount /dev/sda3 /mnt`          | The root partition     |
| 4      | +50%  | 8300 | `/home`     | `mkfs.xfs /dev/sda4`       | `/mnt/home`     | `mount /dev/sda4 /mnt/home`     | The home partition     |
| 5      | +50%  | 8300 | `/mnt/save` | `mkfs.ext4 /dev/sda5`      | `/mnt/mnt/save` | `mount /dev/sda5 /mnt/mnt/save` | The save partition     |


## Update the pacman mirrors

```bash
reflector -c <country> --sort delay --save /etc/pacman.d/mirrorlist -p https
```

## Initialise the pacman keyring

```bash
pacman-key --init && pacman-key --populate archlinux
```

##  Install the base system

Add your GPU driver and processor ucode package.

### Uefi system

```bash
pacstrap /mnt base base-devel linux linux-firmware less networkmanager efivars bootmgr xfsprogs os-prober fish feh pkgfile pacman-contrib archlinux-wallpaper
```

### Bios system

```bash
pacstrap /mnt base base-devel linux linux-firmware networkmanager xfsprogs os-prober fish pkgfile less pacman-contrib feh wallutils archlinux-wallpaper 
```

##  Install the arch dependencies

`paru` it's also required.

### Setup mode

```bash
pacstrap /mnt w3m rustup wget rsync archiso git arch-wiki-lite archinstall man-pages arch-install-scripts aurpublish distrobox toolbox vim
```

### After install mode

```bash
sudo pacman -S --needed w3m rustup git wget rsync git arch-wiki-lite archinstall man-pages arch-install-scripts aurpublish distrobox toolbox vim
```

## Generate fstab

```bash
genfstab -U /mnt >> /mnt/etc/fstab
```

## Enter in the new system

```bash
arch-chroot /mnt && cd ~
```

## Manage accounts

### Create your account

```bash
useradd -m -g wheel -c 'YOUR REAL NAME' -s <shell> <username>
```

### Generate root password

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

## Sign in

```bash
su - <username>
```

### Configure rust

```bash
rustup default stable
```

### Modify the pacman configuration

```bash
sudo vim /etc/pacman.conf
```

### Refresh repositories

```bash
sudo pacman -Sy
```

### Paru

```bash
git clone https://aur.archlinux.org/paru 
cd paru
makepkg -si 
cd ..
rm -rf paru
```

### Arch

#### From GitHub

```bash
git clone https://github.com/otechdo/arch 
cd arch 
make
sudo make install
```

#### From Aur

```bash
paru -Syu manager
```

# Supported desktop

- [@deepin](https://wiki.archlinux.org/title/Deepin_Desktop_Environment)
- [@kde](https://wiki.archlinux.org/title/KDE)
- [@gnome](https://wiki.archlinux.org/title/GNOME)

# Supported window manager

- [@xmonad](https://wiki.archlinux.org/title/xmonad)
- [@i3](https://wiki.archlinux.org/title/i3)

### Run the setup

```bash
arch --setup
```

#### Rebuild a complete arch

```bash
arch --new-config
```

#### Quit the fresh new system

```bash
exit
```

#### Unmounted all partitions

```bash
umount -R /mnt
```

#### Reboot

```bash
reboot
```


# Distrobox management

## Distrobox containers 

Distrobox guests tested successfully with the following container images:

| Distro                | Version                                            | Images                                                                                                                                                                                                                                                                                                                  |
|-----------------------|----------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
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

### List all boxes

```bash
os --list
```

### Create a new box

```bash
os --new
```

### Enter inside a box

```bash
os --use <name>
```

### Stop a box

```bash
os --pause <name>
```

### Stop all boxes

```bash
os --stop
```

### Run a command in a box

```bash
os --run <name> ls
```

### Stop a box

```bash
os --stop <name>
```

### Remove all boxes

```bash
os --clean
```

### Remove a box

```bash
os --remove <name>
```

### Upgrade all boxes

```bash
os --upgrade
```

### Display help

```bash
os --help
```
